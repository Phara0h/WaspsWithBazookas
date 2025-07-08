use axum::{
    extract::State,
    http::{Method, StatusCode},
    response::Json,
    routing::{get, put, delete},
    Router,
};
use chrono::{DateTime, Utc};
use clap::{Arg, Command};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn, error};
use uuid::Uuid;

mod battle;
use battle::BattleParams;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BattlemarkResult {
    id: String,
    timestamp: DateTime<Utc>,
    url: String,
    method: String,
    connections: u32,
    duration: u32,
    threads: u32,
    requests: u64,
    bytes: u64,
    rps: f64,
    status_counts: std::collections::HashMap<i16, u64>,
    latency_p50: Option<u128>,
    latency_p90: Option<u128>,
    latency_p99: Option<u128>,
    raw_output: String,
}





#[derive(Debug, Clone, Serialize, Deserialize)]
struct FireRequest {
    target: String,
    t: Option<u32>,  // threads
    c: Option<u32>,  // connections
    d: Option<u32>,  // duration
    timeout: Option<u32>,
    script: Option<String>,
    method: Option<String>,
    headers: Option<std::collections::HashMap<String, String>>,
    body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FireResponse {
    status: String,
    message: String,
}

struct AppState {
    hive_id: Arc<Mutex<Option<String>>>, // Store the hive-assigned ID
    hostname: String,
    version: String,
    port: u16,
    hive_url: Option<String>,
    hive_token: Option<String>,
    last_battle: Arc<Mutex<Option<BattlemarkResult>>>,
    running: Arc<Mutex<bool>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args = build_cli().get_matches();
    
    let port = *args.get_one::<u16>("port").unwrap_or(&3000);
    let host = args.get_one::<String>("host").unwrap_or(&"127.0.0.1".to_string()).clone();
    let hive_url = args.get_one::<String>("hive-url").map(|s| s.clone());
    let hive_token = args.get_one::<String>("hive-token").map(|s| s.clone());
    
    // Get hostname
    let hostname = hostname::get()
        .unwrap_or_else(|_| "unknown".into())
        .to_string_lossy()
        .to_string();
    
    let version = "2.0.0".to_string();
    
    let addr = format!("{}:{}", host, port);
    
    info!("🐝 Wasp agent starting on {}", addr);
    if let Some(ref hive_url) = &hive_url {
        info!("🏠 Hive URL: {}", hive_url);
    }
    info!("🚀 Ready to launch some rockets!");
    
    let state = Arc::new(AppState {
        hive_id: Arc::new(Mutex::new(None)),
        hostname,
        version,
        port,
        hive_url,
        hive_token,
        last_battle: Arc::new(Mutex::new(None)),
        running: Arc::new(Mutex::new(false)),
    });

    // Start heartbeat if Hive is configured
    if let Some(hive_url) = &state.hive_url {
        let state_clone = state.clone();
        let hive_url = hive_url.clone();
        let hive_token = state.hive_token.clone();
        let port = state.port;
        
        // First check in with the hive
        if let Err(e) = checkin_with_hive(&hive_url, &hive_token, port, &state_clone).await {
            warn!("❌ Failed to check in with hive: {}", e);
        } else {
            // Get the hive ID for logging
            let hive_id_guard = state_clone.hive_id.lock().await;
            if let Some(ref hive_id) = *hive_id_guard {
                info!("✅ Successfully checked in with hive as {}!", hive_id);
            } else {
                info!("✅ Successfully checked in with hive!");
            }
        }
        
        println!("Starting heartbeat loop");
        tokio::spawn(async move {
            heartbeat_loop(state_clone, hive_url, hive_token, port).await;
        });
    }

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    // Build router
    let app = Router::new()
        .route("/boop", get(boop))
        .route("/fire", put(fire))
        .route("/ceasefire", get(ceasefire))
        .route("/die", delete(die))
        .route("/battlereport", get(battlereport))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}



async fn boop() -> &'static str {
    info!("👋 Boop! Someone poked me!");
    "Oh hi"
}

async fn fire(
    State(state): State<Arc<AppState>>,
    Json(request): Json<FireRequest>,
) -> Result<Json<FireResponse>, (StatusCode, String)> {
    let mut running = state.running.lock().await;
    if *running {
        info!("💥 Already firing! Can't launch more rockets right now!");
        return Ok(Json(FireResponse {
            status: "400".to_string(),
            message: "I'm already shooting...".to_string(),
        }));
    }

    // Validate target URL
    if !request.target.starts_with("http://") && !request.target.starts_with("httbs://") {
        info!("❌ Invalid target URL: {}", request.target);
        return Err((StatusCode::BAD_REQUEST, "Invalid target URL".to_string()));
    }

    // Extract parameters with defaults
    let threads = request.t.unwrap_or(10);
    let connections = request.c.unwrap_or(50);
    let duration = request.d.unwrap_or(30);
    
    // Convert headers HashMap to Vec<String> format expected by bench module
    let headers = request.headers
        .as_ref()
        .map(|h| h.iter().map(|(k, v)| format!("{}: {}", k, v)).collect())
        .unwrap_or_default();
    
    let method = request.method.clone().unwrap_or_else(|| "GET".to_string());
    
    let params = BattleParams {
        threads,
        connections,
        duration_secs: duration,
        url: request.target.clone(),
        method: method.clone(),
        headers,
        body: request.body.clone(),
    };
    
    info!("🎯 Target acquired: {}", request.target);
    info!("🔍 Sighting in the target...");
    
    // Do a single request to verify target is available
    match battle::health_check(&params) {
        Ok(_) => {
            info!("✅ Target sighted and locked in!");
        }
        Err(e) => {
            error!("❌ Could not lock in the target: {}", e);
            return Err((StatusCode::BAD_REQUEST, format!("Target health check failed: {}", e)));
        }
    }
    
    info!("🚀 Launching {} rockets at a time with {} rocket launchers for {}s!", connections, threads, duration);
    *running = true;
    
    // Convert headers HashMap to Vec<String> format expected by bench module
    let headers = request.headers
        .as_ref()
        .map(|h| h.iter().map(|(k, v)| format!("{}: {}", k, v)).collect())
        .unwrap_or_default();
    
    let method = request.method.unwrap_or_else(|| "GET".to_string());
    
    let params = BattleParams {
        threads,
        connections,
        duration_secs: duration,
        url: request.target.clone(),
        method: method.clone(),
        headers,
        body: request.body,
    };

    // Run battle in background
    let state_clone = state.clone();
    tokio::spawn(async move {
        match battle::run_battle(params) {
            Ok(result) => {
                info!("🎯 Mission accomplished! {} rockets launched at the rate of {:.2} RPS", result.requests, result.qps);
                info!("📊 Status counts: {:?}", result.status_counts);
                let raw_output = serde_json::to_string(&result).unwrap();
                let battle_result = BattlemarkResult {
                    id: Uuid::new_v4().to_string(),
                    timestamp: Utc::now(),
                    url: request.target,
                    method: method,
                    connections,
                    duration,
                    threads,
                    requests: result.requests,
                    bytes: result.bytes,
                    rps: result.qps,
                    status_counts: result.status_counts,
                    latency_p50: result.latency_p50,
                    latency_p90: result.latency_p90,
                    latency_p99: result.latency_p99,
                    raw_output,
                };
                
                // Store in state
                {
                    let mut last_battle = state_clone.last_battle.lock().await;
                    *last_battle = Some(battle_result.clone());
                }
                
                // Auto-report to hive if configured
                if let Some(hive_url) = &state_clone.hive_url {
                    info!("📡 Sending battle report to hive...");
                    if let Err(e) = report_to_hive_internal(&battle_result, hive_url, &state_clone.hive_token, &state_clone).await {
                        warn!("❌ Failed to send battle report to hive: {}", e);
                    } else {
                        info!("✅ Battle report successfully sent to hive!");
                    }
                }
            }
            Err(e) => {
                error!("💥 Mission failed! {}", e);
            }
        }
        
        // Mark as not running
        let mut running = state_clone.running.lock().await;
        *running = false;
        info!("🛑 Rockets have stopped firing");
    });

    Ok(Json(FireResponse {
        status: "200".to_string(),
        message: "🚀 I'M A'FIRIN' MAH ROCKETS!".to_string(),
    }))
}

async fn ceasefire(State(state): State<Arc<AppState>>) -> Result<Json<FireResponse>, (StatusCode, String)> {
    let mut running = state.running.lock().await;
    if *running {
        info!("🛑 Ceasefire ordered! I stops...");
        *running = false;
        Ok(Json(FireResponse {
            status: "200".to_string(),
            message: "Ok i stops".to_string(),
        }))
    } else {
        info!("🤔 Ceasefire requested but I was not firing 0__0");
        Ok(Json(FireResponse {
            status: "400".to_string(),
            message: "Was not firing 0__0".to_string(),
        }))
    }
}

async fn die(State(state): State<Arc<AppState>>) -> Result<Json<FireResponse>, (StatusCode, String)> {
    let running = state.running.lock().await;
    if *running {
        info!("💀 Can't die while firing rockets!");
        Ok(Json(FireResponse {
            status: "400".to_string(),
            message: "I'm already shooting cant die yet...".to_string(),
        }))
    } else {
        info!("💀 Self-destruct sequence initiated... Goodbye cruel world!");
        
        // Send response first, then exit
        let response = Json(FireResponse {
            status: "200".to_string(),
            message: "💀 Goodbye cruel world!".to_string(),
        });
        
        // Spawn a task to exit after a short delay to ensure response is sent
        tokio::spawn(async {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            std::process::exit(0);
        });
        
        Ok(response)
    }
}

async fn battlereport(State(state): State<Arc<AppState>>) -> Result<Json<BattlemarkResult>, (StatusCode, String)> {
    let last_battle = state.last_battle.lock().await;
    let battle = last_battle.as_ref().ok_or((StatusCode::NOT_FOUND, "No battle results available".to_string()))?;
    Ok(Json(battle.clone()))
}

async fn report_to_hive_internal(
    battle: &BattlemarkResult,
    hive_url: &str,
    hive_token: &Option<String>,
    state: &Arc<AppState>,
) -> anyhow::Result<()> {
    // Send compact battle data to hive (avoiding the large raw_output field)
    let hive_stats = serde_json::json!({
        // Basic stats
        "total_rps": battle.rps,
        "read": battle.bytes,
        "total_requests": battle.requests,
        "tbs": battle.bytes as f64 / 1024.0 / 1024.0 / battle.duration as f64,
        "non_success_requests": battle.status_counts.iter()
            .filter(|(&code, _)| code >= 400)
            .map(|(_, &count)| count)
            .sum::<u64>(),
        "errors": {
            "connect": 0, // We don't track these separately yet
            "read": 0,
            "write": 0,
            "timeout": 0
        },
        "latency": {
            "avg": battle.latency_p50.unwrap_or(0) as f64,
            "max": battle.latency_p99.unwrap_or(0) as f64
        },
        "rps": {
            "avg": battle.rps,
            "max": battle.rps
        },
        // Enhanced verbose data (compact version)
        "status_counts": battle.status_counts.iter()
            .map(|(&code, &count)| (code.to_string(), count))
            .collect::<std::collections::HashMap<String, u64>>(),
        "latency_p50": battle.latency_p50.map(|l| l as f64),
        "latency_p90": battle.latency_p90.map(|l| l as f64),
        "latency_p99": battle.latency_p99.map(|l| l as f64),
        "duration_secs": battle.duration as f64,
        "connections": battle.connections,
        "threads": battle.threads,
        "method": battle.method.clone(),
        "url": battle.url.clone()
        // Removed raw_battle_data to avoid payload size issues
    });
    
    // Get the hive-assigned ID
    let hive_id_guard = state.hive_id.lock().await;
    let hive_id = hive_id_guard.as_ref().ok_or_else(|| {
        anyhow::anyhow!("No hive ID available - wasp may not have checked in properly")
    })?;
    
    let client = reqwest::Client::new();
    let mut request = client
        .put(&format!("{}/wasp/reportin/{}", hive_url, hive_id))
        .header("Content-Type", "application/json")
        .json(&hive_stats);
    
    // Add authorization header if token is provided
    if let Some(token) = hive_token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    
    let response = request.send().await?;
    
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Hive API error: {} - {}", status, error_text));
    }
    
    info!("📡 Successfully reported back to hive as {}", hive_id);
    Ok(())
}

async fn checkin_with_hive(
    hive_url: &str,
    hive_token: &Option<String>,
    port: u16,
    state: &Arc<AppState>,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let mut request = client
        .get(&format!("{}/wasp/checkin/{}", hive_url, port));
    
    // Add authorization header if token is provided
    if let Some(token) = hive_token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    
    let response = request.send().await?;
    
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Hive checkin error: {} - {}", status, error_text));
    }
    
    let checkin_response: serde_json::Value = response.json().await?;
    let hive_id = checkin_response["id"].as_str().unwrap_or("unknown").to_string();
    info!("📡 Checked in with hive, got ID: {}", hive_id);
    
    // Store the hive-assigned ID
    let mut hive_id_guard = state.hive_id.lock().await;
    *hive_id_guard = Some(hive_id);
    
    Ok(())
}

async fn heartbeat_loop(
    state: Arc<AppState>,
    hive_url: String,
    hive_token: Option<String>,
    port: u16,
) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5)); // 5 second heartbeat
    
    info!("💓 Saying hi to hive every 5 seconds");
    
    loop {
        interval.tick().await;
        
        let client = reqwest::Client::new();
        let mut request = client
            .get(&format!("{}/wasp/heartbeat/{}", hive_url, port));
        
        // Add authorization header if token is provided
        if let Some(token) = &hive_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        
        let response = request.send().await;
        
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    //info!("💓 Heartbeat sent successfully");
                } else {
                    warn!("💓 Hive did not wave back correctly with status: {}", resp.status());
                }
            }
            Err(e) => {
                warn!("💓 Failed to say hi to hive: {}", e);
            }
        }
    }
}

fn build_cli() -> Command {
    Command::new("wasp")
        .version("1.0.0")
        .about("Wasp Agent - HTTP Battlemarking Tool")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Port to listen on")
                .default_value("3000")
                .value_parser(clap::value_parser!(u16)),
        )
        .arg(
            Arg::new("host")
                .long("host")
                .help("Host to bind to")
                .default_value("127.0.0.1")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("hive-url")
                .long("hive-url")
                .help("Hive server URL for reporting results")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("hive-token")
                .long("hive-token")
                .help("Hive server authentication token (optional)")
                .value_parser(clap::value_parser!(String)),
        )

} 