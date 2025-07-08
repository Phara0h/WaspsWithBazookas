use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    response::{Html, Json},
    routing::{delete, get, put},
    Router,
};
use chrono::{DateTime, Utc};
use clap::{Arg, Command};

use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing::{info, error, warn};

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Wasp {
    ip: String,
    port: String,
    id: String,
    last_heartbeat: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WaspReport {
    wasp: Wasp,
    status: String,
    stats: Option<BattleStats>,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BattleStats {
    total_rps: f64,
    read: u64,
    total_requests: u64,
    tbs: f64,
    non_success_requests: u64,
    errors: ErrorStats,
    latency: LatencyStats,
    rps: RpsStats,
    // Enhanced verbose reporting fields
    status_counts: Option<std::collections::HashMap<String, u64>>,
    latency_percentiles: Option<LatencyPercentiles>,
    connection_info: Option<ConnectionInfo>,
    timing_info: Option<TimingInfo>,
    raw_benchmark_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ErrorStats {
    connect: u64,
    read: u64,
    write: u64,
    timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LatencyStats {
    avg: f64,
    max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RpsStats {
    avg: f64,
    max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LatencyPercentiles {
    p50: f64,
    p75: f64,
    p90: f64,
    p95: f64,
    p99: f64,
    p99_9: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConnectionInfo {
    total_connections: u32,
    active_connections: u32,
    connection_errors: u64,
    connection_timeouts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TimingInfo {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    duration_seconds: f64,
    warmup_time: Option<f64>,
    cooldown_time: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PokeRequest {
    target: String,
    #[serde(default, deserialize_with = "deserialize_optional_u32")]
    t: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_optional_u32")]
    c: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_optional_u32")]
    d: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_optional_u32")]
    timeout: Option<u32>,
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    headers: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RunningStatus {
    running: RunningInfo,
    percent: String,
    eta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RunningInfo {
    target: String,
    threads: u32,
    concurrency: u32,
    duration: u32,
    timeout: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Report {
    target: String,
    threads: u32,
    concurrency: u32,
    duration: u32,
    timeout: u32,
    start_time: DateTime<Utc>,
    wasp: WaspReports,
    status: ReportStatus,
    latency: ReportLatency,
    rps: ReportRps,
    total_rps: f64,
    total_requests: u64,
    read: ReadData,
    tbs: ReadData,
    non_success_requests: u64,
    errors: ErrorStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WaspReports {
    reports: Vec<WaspReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReportStatus {
    completed: u32,
    failed: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReportLatency {
    avg: f64,
    max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReportRps {
    avg: f64,
    max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReadData {
    val: f64,
    unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LocalWasp {
    ip: String,
    port: String,
    pid: u32,
}

struct AppState {
    wasps: Arc<Mutex<Vec<Wasp>>>,
    running: Arc<Mutex<bool>>,
    run_timeout: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    duration: Arc<Mutex<u64>>,
    wasp_done_count: Arc<Mutex<u32>>,
    wasps_running_count: Arc<Mutex<u32>>,
    run_timestamp: Arc<Mutex<f64>>,
    id_count: Arc<Mutex<u32>>,
    report: Arc<Mutex<Option<Report>>>,
    log_path: Option<String>,
    report_generated: Arc<Mutex<bool>>, // Flag to prevent multiple gen_report calls
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Get current timestamp in seconds since epoch
fn get_current_time() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

/// Find a wasp by IP and port
fn find_wasp(wasps: &[Wasp], ip: &str, port: &str) -> Option<usize> {
    wasps.iter().position(|w| w.ip == ip && w.port == port)
}

/// Check if the hive is currently running
async fn is_running(state: &Arc<AppState>) -> bool {
    *state.running.lock().await
}

/// Set the running state
async fn set_running_state(state: &Arc<AppState>, running: bool) {
    *state.running.lock().await = running;
}

/// Get wasp count
async fn get_wasp_count(state: &Arc<AppState>) -> usize {
    state.wasps.lock().await.len()
}

/// Create HTTP client for wasp communication
fn create_wasp_client() -> reqwest::Client {
    reqwest::Client::new()
}

/// Send HTTP request to a wasp
async fn send_wasp_request(url: &str, method: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = create_wasp_client();
    match method {
        "GET" => client.get(url).send().await,
        "DELETE" => client.delete(url).send().await,
        _ => {
            // For unsupported methods, just return a GET request as fallback
            client.get(url).send().await
        }
    }
}

/// Kill a process by PID
fn kill_process(pid: u32) -> Result<(), std::io::Error> {
    std::process::Command::new("kill")
        .arg(pid.to_string())
        .output()
        .map(|_| ())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

/// Update wasp heartbeat
async fn update_wasp_heartbeat(state: &Arc<AppState>, ip: &str, port: &str) -> bool {
    let mut wasps = state.wasps.lock().await;
    if let Some(found) = find_wasp(&wasps, ip, port) {
        if let Some(wasp) = wasps.get_mut(found) {
            wasp.last_heartbeat = get_current_time();
            return true;
        }
    }
    false
}

/// Mark wasp as offline
async fn mark_wasp_offline(state: &Arc<AppState>, ip: &str, port: &str) {
    let mut wasps = state.wasps.lock().await;
    if let Some(found) = find_wasp(&wasps, ip, port) {
        if let Some(wasp) = wasps.get_mut(found) {
            wasp.last_heartbeat = 0.0;
        }
    }
}

/// Check if wasp is online (heartbeat within 15 seconds)
fn is_wasp_online(wasp: &Wasp) -> bool {
    (get_current_time() - wasp.last_heartbeat) < 15.0
}

/// Deserialize optional u32 from either string or number
fn deserialize_optional_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrU32 {
        String(String),
        U32(u32),
    }
    
    match Option::<StringOrU32>::deserialize(deserializer)? {
        Some(StringOrU32::String(s)) => s.parse::<u32>().map(Some).map_err(serde::de::Error::custom),
        Some(StringOrU32::U32(n)) => Ok(Some(n)),
        None => Ok(None),
    }
}

// ============================================================================
// Main Application
// ============================================================================

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = build_cli().get_matches();
    
    let port = *args.get_one::<u16>("port").unwrap_or(&4269);
    let host = args.get_one::<String>("host").unwrap_or(&"0.0.0.0".to_string()).clone();
    let log_path = args.get_one::<String>("log").map(|s| s.clone());
    
    setup_logging(log_path.clone())?;
    
    let addr = format!("{}:{}", host, port);
    info!("🏠 Hive server starting on {}", addr);
    info!("🐝 Ready to coordinate the wasps!");
    
    let state = Arc::new(AppState {
        wasps: Arc::new(Mutex::new(Vec::new())),
        running: Arc::new(Mutex::new(false)),
        run_timeout: Arc::new(Mutex::new(None)),
        duration: Arc::new(Mutex::new(0)),
        wasp_done_count: Arc::new(Mutex::new(0)),
        wasps_running_count: Arc::new(Mutex::new(0)),
        run_timestamp: Arc::new(Mutex::new(0.0)),
        id_count: Arc::new(Mutex::new(0)),
        report: Arc::new(Mutex::new(None)),
        log_path,
        report_generated: Arc::new(Mutex::new(false)),
    });

    // Start health check loop
    let state_clone = state.clone();
    tokio::spawn(async move {
        health_check_loop(state_clone).await;
    });

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    // Build router
    let app = Router::new()
        .route("/", get(index))
        .route("/wasp/heartbeat/:port", get(wasp_heartbeat))
        .route("/wasp/checkin/:port", get(wasp_checkin))
        .route("/wasp/list", get(wasp_list))
        .route("/wasp/boop/snoots", get(boop_snoots))
        .route("/wasp/reportin/:id", put(wasp_reportin))
        .route("/wasp/reportin/:id/failed", put(wasp_reportin_failed))
        .route("/hive/ceasefire", get(hive_ceasefire))
        .route("/hive/poke", put(hive_poke))
        .route("/hive/torch", delete(hive_torch))
        .route("/hive/status", get(hive_status))
        .route("/hive/status/done", get(hive_status_done))
        .route("/hive/status/report", get(hive_status_report))
        .route("/hive/status/report/:val", get(hive_status_report_val))

        .route("/hive/spawn/local/:amount", get(hive_spawn_local))
        .nest_service("/client", ServeDir::new("src/client"))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ============================================================================
// Logging Setup
// ============================================================================

fn setup_logging(log_path: Option<String>) -> anyhow::Result<()> {
    if let Some(log_path) = log_path {
        let log_file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;
        
        tracing_subscriber::fmt()
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_ansi(false)
            .with_writer(log_file)
            .init();
        info!("Logging to file: {}", log_path);
    } else {
        tracing_subscriber::fmt::init();
    }
    Ok(())
}

// ============================================================================
// Route Handlers
// ============================================================================

async fn index() -> Html<&'static str> {
    Html(include_str!("client/index.html"))
}

async fn wasp_heartbeat(
    State(state): State<Arc<AppState>>,
    Path(port): Path<String>,
) -> StatusCode {
    // For now, use a default IP since we can't get the client IP easily
    let ip = "127.0.0.1".to_string();
    
    if update_wasp_heartbeat(&state, &ip, &port).await {
        StatusCode::OK
    } else {
        info!("A random wasp is reporting a heartbeat that is not part of the hive!");
        StatusCode::BAD_REQUEST
    }
}

async fn wasp_checkin(
    State(state): State<Arc<AppState>>,
    Path(port): Path<String>,
) -> Json<serde_json::Value> {
    // For now, use a default IP since we can't get the client IP easily
    let ip = "127.0.0.1".to_string();
    
    // Check if wasp already exists first
    let existing_wasp = {
        let wasps = state.wasps.lock().await;
        wasps.iter().find(|w| w.ip == ip && w.port == port).cloned()
    };
    
    if let Some(existing_wasp) = existing_wasp {
        // Update heartbeat for existing wasp
        {
            let mut wasps = state.wasps.lock().await;
            if let Some(found) = find_wasp(&wasps, &ip, &port) {
                wasps[found].last_heartbeat = get_current_time();
            }
        }
        
        info!("Wasp {} re-checking in at {}!", existing_wasp.id, ip);
        Json(serde_json::json!({ "id": existing_wasp.id }))
    } else {
        // Create new wasp
        let mut wasps = state.wasps.lock().await;
        let mut id_count = state.id_count.lock().await;
        
        let wasp = Wasp {
            ip: ip.clone(),
            port: port.clone(),
            id: format!("BuzzyBoi{}", *id_count),
            last_heartbeat: get_current_time(),
        };
        
        wasps.push(wasp.clone());
        *id_count += 1;
        
        info!("New wasp {} checking in at {}!", wasp.id, ip);
        info!("Total Wasps: {}", wasps.len());
        
        Json(serde_json::json!({ "id": wasp.id }))
    }
}

async fn wasp_list(State(state): State<Arc<AppState>>) -> Json<Vec<Wasp>> {
    let wasps = state.wasps.lock().await;
    Json(wasps.clone())
}

async fn boop_snoots(State(state): State<Arc<AppState>>) -> Result<Json<String>, (StatusCode, String)> {
    if is_running(&state).await {
        return Err((StatusCode::BAD_REQUEST, "Already running".to_string()));
    }
    // Clone wasps before any await
    let wasps = {
        let wasps_guard = state.wasps.lock().await;
        wasps_guard.clone()
    };
    if wasps.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "There are no wasps to boop.".to_string()));
    }
    info!("Booping the snoots of the buzzy bois");
    let num_wasps = wasps.len();
    for wasp in wasps.iter() {
        let url = format!("http://{}:{}/boop", wasp.ip, wasp.port);
        match send_wasp_request(&url, "GET").await {
            Ok(_) => {
                // Request successful
            }
            Err(e) => {
                mark_wasp_offline(&state, &wasp.ip, &wasp.port).await;
                error!("Error booping wasp: {}", e);
            }
        }
    }
    check_health_status(&state).await;
    info!("Total Wasps: {}", num_wasps);
    Ok(Json(format!("Hive is operational with {} wasps ready and waiting orders.", num_wasps)))
}

async fn wasp_reportin(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(stats_data): Json<serde_json::Value>,
) -> Result<StatusCode, (StatusCode, String)> {
    if !is_running(&state).await {
        return Err((StatusCode::PRECONDITION_FAILED, "Not running".to_string()));
    }
    
    // Extract wasp data first, then drop the lock
    let wasp = {
        let wasps = state.wasps.lock().await;
        wasps.iter().find(|w| w.id == id).cloned()
    };
    
    if let Some(wasp) = wasp {
        // Extract basic stats from data
        let total_rps = stats_data.get("total_rps").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let total_requests = stats_data.get("total_requests").and_then(|v| v.as_u64()).unwrap_or(0);
        let bytes = stats_data.get("read").and_then(|v| v.as_u64()).unwrap_or(0);
        let tbs = stats_data.get("tbs").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let non_success_requests = stats_data.get("non_success_requests").and_then(|v| v.as_u64()).unwrap_or(0);
        
        // Extract error stats
        let connect_errors = stats_data.get("errors")
            .and_then(|e| e.get("connect"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let read_errors = stats_data.get("errors")
            .and_then(|e| e.get("read"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let write_errors = stats_data.get("errors")
            .and_then(|e| e.get("write"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let timeout_errors = stats_data.get("errors")
            .and_then(|e| e.get("timeout"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        
        // Extract latency stats
        let avg_latency = stats_data.get("latency")
            .and_then(|l| l.get("avg"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let max_latency = stats_data.get("latency")
            .and_then(|l| l.get("max"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        
        // Extract RPS stats
        let avg_rps = stats_data.get("rps")
            .and_then(|r| r.get("avg"))
            .and_then(|v| v.as_f64())
            .unwrap_or(total_rps);
        let max_rps = stats_data.get("rps")
            .and_then(|r| r.get("max"))
            .and_then(|v| v.as_f64())
            .unwrap_or(total_rps);
        
        // Create enhanced benchmark stats
        let mut enhanced_stats = BattleStats {
            total_rps,
            read: bytes,
            total_requests,
            tbs,
            non_success_requests,
            errors: ErrorStats { 
                connect: connect_errors, 
                read: read_errors, 
                write: write_errors, 
                timeout: timeout_errors 
            },
            latency: LatencyStats { avg: avg_latency, max: max_latency },
            rps: RpsStats { avg: avg_rps, max: max_rps },
            status_counts: None,
            latency_percentiles: None,
            connection_info: None,
            timing_info: None,
            raw_benchmark_data: Some(stats_data.clone()),
        };
        
        // Extract status counts if available
        if let Some(status_counts) = stats_data.get("status_counts") {
            if let Some(counts_map) = status_counts.as_object() {
                let mut counts = std::collections::HashMap::new();
                for (code, count) in counts_map {
                    if let Some(count_val) = count.as_u64() {
                        counts.insert(code.clone(), count_val);
                    }
                }
                enhanced_stats.status_counts = Some(counts);
            }
        }
        
        // Extract latency percentiles if available
        if let Some(latency_p50) = stats_data.get("latency_p50").and_then(|v| v.as_f64()) {
            let latency_p90 = stats_data.get("latency_p90").and_then(|v| v.as_f64()).unwrap_or(latency_p50);
            let latency_p99 = stats_data.get("latency_p99").and_then(|v| v.as_f64()).unwrap_or(latency_p90);
            
            enhanced_stats.latency_percentiles = Some(LatencyPercentiles {
                p50: latency_p50,
                p75: latency_p50, // Default to p50 if not available
                p90: latency_p90,
                p95: latency_p90, // Default to p90 if not available
                p99: latency_p99,
                p99_9: latency_p99, // Default to p99 if not available
            });
            
            // Update basic latency stats with percentile data
            enhanced_stats.latency.avg = latency_p50;
            enhanced_stats.latency.max = latency_p99;
        }
        
        // Extract connection info if available
        if let Some(connections) = stats_data.get("connections").and_then(|v| v.as_u64()) {
            let _threads = stats_data.get("threads").and_then(|v| v.as_u64()).unwrap_or(connections);
            enhanced_stats.connection_info = Some(ConnectionInfo {
                total_connections: connections as u32,
                active_connections: connections as u32,
                connection_errors: 0, // Could be extracted if available
                connection_timeouts: 0, // Could be extracted if available
            });
        }
        
        // Extract timing info if available
        if let Some(duration) = stats_data.get("duration_secs").and_then(|v| v.as_f64()) {
            enhanced_stats.timing_info = Some(TimingInfo {
                start_time: Utc::now() - chrono::Duration::seconds(duration as i64),
                end_time: Utc::now(),
                duration_seconds: duration,
                warmup_time: None,
                cooldown_time: None,
            });
        }
        
        info!("📊 Received enhanced report from wasp {}: {} requests, {:.2} RPS", 
              wasp.id, total_requests, total_rps);
        
        // Log enhanced data if available
        if let Some(ref status_counts) = enhanced_stats.status_counts {
            info!("📈 Status counts: {:?}", status_counts);
        }
        if let Some(ref percentiles) = enhanced_stats.latency_percentiles {
            info!("⏱️ Latency percentiles - P50: {:.2}μs, P90: {:.2}μs, P99: {:.2}μs", 
                  percentiles.p50, percentiles.p90, percentiles.p99);
        }
        
        // Now update the report and counters - only hold one lock at a time
        let should_stop = {
            let mut wasp_done_count = state.wasp_done_count.lock().await;
            *wasp_done_count += 1;
            let current_done = *wasp_done_count;
            
            // Drop wasp_done_count lock before acquiring report lock
            drop(wasp_done_count);
            
            let mut report = state.report.lock().await;
            if let Some(ref mut report) = *report {
                report.status.completed += 1;
                
                report.wasp.reports.push(WaspReport {
                    wasp: wasp.clone(),
                    status: "complete".to_string(),
                    stats: Some(enhanced_stats),
                    error: None,
                });
                
                report.total_rps += total_rps;
                report.read.val += bytes as f64;
                report.total_requests += total_requests;
                report.tbs.val += tbs;
                report.errors.connect += connect_errors;
                report.errors.read += read_errors;
                report.errors.write += write_errors;
                report.errors.timeout += timeout_errors;
                report.non_success_requests += non_success_requests;
            }
            
            // Drop report lock before checking if we should stop
            drop(report);
            
            // Check if all wasps have reported
            let wasps_running_count = state.wasps_running_count.lock().await;
            current_done >= *wasps_running_count
        };
        
        if should_stop {
            // All wasps have reported back, we can stop
            set_running_state(&state, false).await;
            gen_report(&state).await;
        }
        
        Ok(StatusCode::OK)
    } else {
        Err((StatusCode::PRECONDITION_FAILED, "Wasp not found".to_string()))
    }
}

async fn wasp_reportin_failed(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(error): Json<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    if !is_running(&state).await {
        return Err((StatusCode::PRECONDITION_FAILED, "Not running".to_string()));
    }
    
    // Extract wasp data first, then drop the lock
    let wasp = {
        let wasps = state.wasps.lock().await;
        wasps.iter().find(|w| w.id == id).cloned()
    };
    
    if let Some(wasp) = wasp {
        // Now update the report and counters - only hold one lock at a time
        let should_stop = {
            let mut wasp_done_count = state.wasp_done_count.lock().await;
            *wasp_done_count += 1;
            let current_done = *wasp_done_count;
            
            // Drop wasp_done_count lock before acquiring report lock
            drop(wasp_done_count);
            
            let mut report = state.report.lock().await;
            if let Some(ref mut report) = *report {
                report.wasp.reports.push(WaspReport {
                    wasp: wasp.clone(),
                    status: "failed".to_string(),
                    stats: None,
                    error: Some(error),
                });
                report.status.failed += 1;
            }
            
            // Drop report lock before checking if we should stop
            drop(report);
            
            // Check if all wasps have reported
            let wasps_running_count = state.wasps_running_count.lock().await;
            current_done >= *wasps_running_count
        };
        
        if should_stop {
            // All wasps have reported back, we can stop
            set_running_state(&state, false).await;
            gen_report(&state).await;
        }
        
        Ok(StatusCode::OK)
    } else {
        Err((StatusCode::PRECONDITION_FAILED, "Wasp not found".to_string()))
    }
}


async fn hive_ceasefire(State(state): State<Arc<AppState>>) -> Result<Json<String>, (StatusCode, String)> {
    if !is_running(&state).await {
        return Err((StatusCode::BAD_REQUEST, "They are already idle.".to_string()));
    }
    
    let wasps = state.wasps.lock().await;
    for wasp in wasps.iter() {
        let url = format!("http://{}:{}/ceasefire", wasp.ip, wasp.port);
        
        if let Err(e) = send_wasp_request(&url, "GET").await {
            error!("Error sending ceasefire to wasp: {}", e);
        }
    }
    
    set_running_state(&state, false).await;
    Ok(Json("Ceasefire ordered!".to_string()))
}

async fn hive_poke(
    State(state): State<Arc<AppState>>,
    Json(request): Json<PokeRequest>,
) -> Result<Json<String>, (StatusCode, String)> {
    if is_running(&state).await {
        return Err((StatusCode::BAD_REQUEST, "Already running".to_string()));
    }
    
    // Extract wasps data first, then drop the lock
    let (wasps, wasp_count) = {
        let wasps = state.wasps.lock().await;
        if wasps.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "No wasps available".to_string()));
        }
        (wasps.clone(), wasps.len())
    };
    
    let duration = request.d.unwrap_or(30);
    set_running_state(&state, true).await;
    
    // Reset counters - only hold one lock at a time
    *state.wasp_done_count.lock().await = 0;
    *state.wasps_running_count.lock().await = wasp_count as u32;
    *state.run_timestamp.lock().await = get_current_time();
    *state.duration.lock().await = duration as u64;
    *state.report_generated.lock().await = false; // Reset flag for new benchmark
    
    // Create initial report
    let report = Report {
        target: request.target.clone(),
        threads: request.t.unwrap_or(10),
        concurrency: request.c.unwrap_or(50),
        duration,
        timeout: request.timeout.unwrap_or(2),
        start_time: Utc::now(),
        wasp: WaspReports { reports: Vec::new() },
        status: ReportStatus { completed: 0, failed: 0 },
        latency: ReportLatency { avg: 0.0, max: 0.0 },
        rps: ReportRps { avg: 0.0, max: 0.0 },
        total_rps: 0.0,
        total_requests: 0,
        read: ReadData { val: 0.0, unit: "bytes".to_string() },
        tbs: ReadData { val: 0.0, unit: "MB/s".to_string() },
        non_success_requests: 0,
        errors: ErrorStats { connect: 0, read: 0, write: 0, timeout: 0 },
    };
    *state.report.lock().await = Some(report);
    
    // Send fire command to all wasps
    for wasp in wasps.iter() {
        let url = format!("http://{}:{}/fire", wasp.ip, wasp.port);
        let client = create_wasp_client();
        
        if let Err(e) = client.put(&url).json(&request).send().await {
            error!("Error sending fire command to wasp: {}", e);
        }
    }
    
    // Set timeout to check if all reports are in, but don't automatically stop
    let state_clone = state.clone();
    let timeout_handle = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(duration as u64)).await;
        
        // Wait for a grace period for late wasps
        tokio::time::sleep(Duration::from_secs(3)).await;
        
        // Check if all wasps have reported back - only hold one lock at a time
        let should_stop = {
            let wasp_done_count = state_clone.wasp_done_count.lock().await;
            let current_done = *wasp_done_count;
            drop(wasp_done_count);
            
            let wasps_running_count = state_clone.wasps_running_count.lock().await;
            current_done >= *wasps_running_count
        };
        
        if should_stop {
            // All reports are in, we can stop
            set_running_state(&state_clone, false).await;
            gen_report(&state_clone).await;
        } else {
            // Not all reports are in, but we've hit the timeout (with grace period)
            let wasp_done_count = state_clone.wasp_done_count.lock().await;
            let wasps_running_count = state_clone.wasps_running_count.lock().await;
            warn!("Timeout (with grace period) reached but only {}/{} wasps have reported back", 
                  *wasp_done_count, *wasps_running_count);
            set_running_state(&state_clone, false).await;
            gen_report(&state_clone).await;
        }
    });
    
    *state.run_timeout.lock().await = Some(timeout_handle);
    
    Ok(Json("Attack launched!".to_string()))
}



async fn hive_torch(State(state): State<Arc<AppState>>)-> Result<Json<String>, (StatusCode, String)> {
    // Clone the wasp list and drop the lock before spawning tasks
    let wasps: Vec<_> = {
        let wasps_guard = state.wasps.lock().await;
        wasps_guard.clone()
    };
    let count = wasps.len();
    
    info!("R.I.P All {} wasps. :'(", count);
    
    // Create a client with reasonable timeout for individual requests
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());
    
    // Send kill commands to all wasps concurrently
    let mut kill_tasks = Vec::new();
    for wasp in wasps.iter() {
        let url = format!("http://{}:{}/die", wasp.ip, wasp.port);
        let client = client.clone();
        
        let task = tokio::spawn(async move {
            match client.delete(&url).send().await {
                Ok(_) => {
                    info!("Successfully sent kill command to wasp at {}", url);
                    Ok(())
                }
                Err(e) => {
                    // Don't log errors for expected connection failures when wasps exit
                    if !e.is_timeout() && !e.is_connect() {
                        error!("Error killing wasp at {}: {}", url, e);
                    }
                    Err(e)
                }
            }
        });
        kill_tasks.push(task);
    }
    
    // Wait for all kill tasks to complete (no arbitrary timeout)
    for task in kill_tasks {
        let _ = task.await;
    }
    
    info!("All kill commands sent");
    
    // Now reacquire the lock and clear the list
    let mut wasps_guard = state.wasps.lock().await;
    wasps_guard.clear();
    
    Ok(Json(format!("R.I.P All {} wasps. :'(", count)))
}

async fn hive_status_done(State(state): State<Arc<AppState>>) -> Result<Json<String>, (StatusCode, String)> {
    if is_running(&state).await {
        return Err((StatusCode::BAD_REQUEST, "Still running".to_string()));
    }
    
    Ok(Json("done".to_string()))
}

async fn hive_status_report(State(state): State<Arc<AppState>>) -> Result<Json<Report>, (StatusCode, String)> {
    if is_running(&state).await {
        return Err((StatusCode::BAD_REQUEST, "Still running".to_string()));
    }
    
    let report = state.report.lock().await;
    if let Some(ref report) = *report {
        Ok(Json(report.clone()))
    } else {
        Err((StatusCode::BAD_REQUEST, "No report yet.".to_string()))
    }
}

async fn hive_status_report_val(
    State(state): State<Arc<AppState>>,
    Path(val): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if is_running(&state).await {
        return Err((StatusCode::BAD_REQUEST, "Still running".to_string()));
    }
    
    let report = state.report.lock().await;
    if let Some(ref report) = *report {
        let report_json = serde_json::to_value(report).unwrap();
        if let Some(field_value) = report_json.get(&val) {
            Ok(Json(field_value.clone()))
        } else {
            Err((StatusCode::BAD_REQUEST, "No hive information on that.".to_string()))
        }
    } else {
        Err((StatusCode::BAD_REQUEST, "No report yet.".to_string()))
    }
}

async fn hive_status(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if is_running(&state).await {
        // Extract all needed data with minimal lock scope
        let (target, threads, concurrency, duration_val, timeout) = {
            let report_guard = state.report.lock().await;
            if let Some(report) = &*report_guard {
                (
                    report.target.clone(),
                    report.threads,
                    report.concurrency,
                    report.duration,
                    report.timeout,
                )
            } else {
                return Err((StatusCode::BAD_REQUEST, "No report yet.".to_string()));
            }
        };
        let run_timestamp = *state.run_timestamp.lock().await;
        let duration = *state.duration.lock().await;

        let current_time = get_current_time();
        let elapsed = current_time - run_timestamp;
        let percent = if duration > 0 {
            ((elapsed / duration as f64) * 100.0).round() as u32
        } else {
            0
        };
        let eta = if duration > elapsed as u64 {
            (duration as f64 - elapsed) as u64
        } else {
            0
        };

        let status = serde_json::json!({
            "running": {
                "target": target,
                "threads": threads,
                "concurrency": concurrency,
                "duration": duration_val,
                "timeout": timeout,
            },
            "percent": format!("{}%", percent),
            "eta": format!("{} seconds", eta),
        });
        return Ok(Json(status));
    }

    let wasp_count = get_wasp_count(&state).await;
    Ok(Json(serde_json::json!(format!("Hive is operational with {} wasps ready and waiting orders.", wasp_count))))
}

async fn hive_spawn_local(
    State(state): State<Arc<AppState>>,
    Path(amount): Path<u32>,
) -> Result<Json<String>, (StatusCode, String)> {
    if is_running(&state).await {
        return Err((StatusCode::BAD_REQUEST, "Still running".to_string()));
    }
    info!("Starting {} Wasps...", amount);
    
    // Find available ports starting from 3001
    let mut next_port = 3001;
    
    for i in 0..amount {
        // Find an available port
        while next_port < 3100 {
            // Check if port is available by trying to bind to it
            if let Ok(_) = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", next_port)).await {
                break;
            }
            next_port += 1;
        }
        
        if next_port >= 3100 {
            error!("No available ports found for wasp {}", i);
            continue;
        }
        
        let hive_url = format!("http://127.0.0.1:{}", 4269);
        let wasp_port = next_port.to_string();
        
        let child = std::process::Command::new("wasp")
            .args(&["--hive-url", &hive_url, "--port", &wasp_port])
            .spawn();
            
        match child {
            Ok(_child) => {
                // Don't create the wasp record here - let the wasp check in and get its own ID
                info!("Started local wasp on port {}", next_port);
            }
            Err(e) => {
                error!("Failed to start wasp on port {}: {}", next_port, e);
            }
        }
        
        // Small delay to avoid port conflicts
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        next_port += 1;
    }
    
    Ok(Json(format!("Started {} local wasps (they will check in automatically)", amount)))
}

// ============================================================================
// Background Tasks
// ============================================================================



async fn gen_report(state: &Arc<AppState>) {
    let mut report_generated = state.report_generated.lock().await;
    if *report_generated {
        info!("🔄 gen_report called again but already generated - ignoring");
        return; // Prevent multiple calls
    }
    *report_generated = true;
    info!("📊 Generating final report...");

    let report = state.report.lock().await;
    if let Some(ref report) = *report {
        info!("Benchmark completed!");
        info!("Target: {}", report.target);
        info!("Total RPS: {:.2}", report.total_rps);
        info!("Total Requests: {}", report.total_requests);
        info!("Completed: {}, Failed: {}", report.status.completed, report.status.failed);
    }
}

async fn check_health_status(state: &Arc<AppState>) {
    let mut wasps = state.wasps.lock().await;
    
    wasps.retain(|wasp| {
        let is_online = is_wasp_online(wasp);
        if !is_online {
            info!("Wasp {} at {}:{} is offline", wasp.id, wasp.ip, wasp.port);
        }
        is_online
    });
    
    info!("Health check complete. {} wasps online", wasps.len());
}

async fn health_check_loop(state: Arc<AppState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        check_health_status(&state).await;
    }
}

// ============================================================================
// CLI Configuration
// ============================================================================

fn build_cli() -> Command {
    Command::new("hive")
        .version("1.0.0")
        .about("Hive Server - Wasp Coordination Tool")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Port to listen on")
                .default_value("4269")
                .value_parser(clap::value_parser!(u16)),
        )
        .arg(
            Arg::new("host")
                .long("host")
                .help("Host to bind to")
                .default_value("0.0.0.0")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("log")
                .long("log")
                .help("Log file path")
                .value_parser(clap::value_parser!(String)),
        )
} 