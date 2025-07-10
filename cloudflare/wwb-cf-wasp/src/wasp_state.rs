use worker::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::types::{BattleState, BazookaRequest, BazookaCommand};
use wasm_bindgen_futures::spawn_local;
use uuid::Uuid;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaspDurableState {
    pub wasp_id: String,
    pub port: String,
    pub hive_url: String,
    pub hive_token: Option<String>,
    pub bazooka_worker_url: String,
    pub wasp_worker_url: String,
    pub heartbeat_active: bool,
    pub battle_state: Option<BattleState>,
    pub next_bazooka_id: u32,
}

impl WaspDurableState {
    pub fn new(wasp_id: String, port: String, hive_url: String, hive_token: Option<String>, bazooka_worker_url: String, wasp_worker_url: String) -> Self {
        Self {
            wasp_id,
            port,
            hive_url,
            hive_token,
            bazooka_worker_url,
            wasp_worker_url,
            heartbeat_active: false,
            battle_state: None,
            next_bazooka_id: 1,
        }
    }

    pub fn initialize_battle(&mut self, fire_request: &crate::types::FireRequest) -> Result<()> {
        let battle_id = format!("battle_{}", Uuid::new_v4().to_string());
        let start_time = Date::now().as_millis() as u64; // milliseconds since epoch
        console_log!("Initializing battle: {:?}, start_time: {:?}, battle_id: {:?}", fire_request , start_time,battle_id);
        let battle_state = BattleState {
            battle_id: battle_id.clone(),
            start_time,
            duration_secs: fire_request.d.unwrap_or(30),
            target: fire_request.target.clone(),
            method: fire_request.method.clone().unwrap_or_else(|| "GET".to_string()),
            headers: fire_request.headers.clone().unwrap_or_default(),
            body: fire_request.body.clone(),
            timeout: fire_request.timeout.unwrap_or(2),
            total_connections: fire_request.c.unwrap_or(50),
            active_bazookas: 0,
            completed_bazookas: 0,
            total_requests: 0,
            total_bytes: 0,
            total_rps: 0.0,
            status_counts: HashMap::new(),
            latency_p50: None,
            latency_p90: None,
            latency_p99: None,
            is_running: true,
        };

        self.battle_state = Some(battle_state);
        self.next_bazooka_id = 1;
        Ok(())
    }

    pub fn spawn_bazooka(&mut self) -> Result<BazookaRequest> {
        let battle_state = self.battle_state.as_mut()
            .ok_or_else(|| Error::RustError("No active battle".to_string()))?;

        if !battle_state.is_running {
            return Err(Error::RustError("Battle is not running".to_string()));
        }

        let bazooka_id = self.next_bazooka_id;
        self.next_bazooka_id += 1;

        battle_state.active_bazookas += 1;

        let bazooka_request = BazookaRequest {
            id:bazooka_id,
            target: battle_state.target.clone(),
            method: battle_state.method.clone(),
            headers: battle_state.headers.clone(),
            body: battle_state.body.clone(),
            timeout: battle_state.timeout,
            battle_id: battle_state.battle_id.clone(),
            battle_start_time: battle_state.start_time,
            battle_duration_secs: battle_state.duration_secs,
            callback_url: format!("{}/bazooka_completed", self.wasp_worker_url),
        };

        Ok(bazooka_request)
    }

    pub async fn handle_bazooka_completed(&mut self, command: BazookaCommand, state: & State) -> Result<Option<BazookaRequest>> {
        let battle_state = self.battle_state.as_mut()
            .ok_or_else(|| Error::RustError("No active battle".to_string()))?;

        if !battle_state.is_running {
            console_log!("Battle is not running");
            return Ok(None);
        }

        // Update battle stats with bazooka results
         let result = command.result.unwrap();
            battle_state.total_requests += result.requests_completed;
            battle_state.total_bytes += result.total_bytes;
            
            // Aggregate status counts
            for (status, count) in result.status_counts {
                *battle_state.status_counts.entry(status).or_insert(0) += count;
            }

            // Update latency percentiles (simplified - in real implementation you'd aggregate properly)
            if let Some(latency) = result.latency_p50 {
                battle_state.latency_p50 = Some(latency);
            }
            if let Some(latency) = result.latency_p90 {
                battle_state.latency_p90 = Some(latency);
            }
            if let Some(latency) = result.latency_p99 {
                battle_state.latency_p99 = Some(latency);
            }
        

        battle_state.active_bazookas -= 1;
        battle_state.completed_bazookas += 1;
        

        // Check if battle duration has elapsed
        let current_time = Date::now().as_millis() as u64; // milliseconds since epoch
        
        let elapsed = current_time - battle_state.start_time;
        if result.rps == 0.0 { // convert seconds to milliseconds
            // Battle is complete
            if battle_state.active_bazookas == 0 {
                console_log!("Battle is complete");
                battle_state.is_running = false;
            }
            state.storage().put("wasp_state", self.clone()).await?;
            return Ok(None);
        }

        console_log!("Bazooka completed, active: {}, completed: {}, total_connections: {}", battle_state.active_bazookas, battle_state.completed_bazookas, battle_state.total_connections);

        // Check if we should spawn another bazooka
        if battle_state.is_running {
            console_log!("Spawning another bazooka");
        
            // Spawn another bazooka
           let bazooka_request = self.spawn_bazooka().map(Some);
            state.storage().put("wasp_state", self.clone()).await?;
            return bazooka_request;

        } else {
            state.storage().put("wasp_state", self.clone()).await?;
            // All bazookas spawned, wait for completion
            Ok(None)
        }
    }

    pub fn get_battle_report(&self) -> Result<serde_json::Value> {
        let battle_state = self.battle_state.as_ref()
            .ok_or_else(|| Error::RustError("No battle state available".to_string()))?;

        // Calculate RPS
        let duration = battle_state.duration_secs as f64;
        let total_rps = if duration > 0.0 {
            battle_state.total_requests as f64 / duration
        } else {
            0.0
        };

        // Format report for hive (matching the format from the original wasp.rs)
        let report = serde_json::json!({
            // Basic stats
            "total_rps": total_rps,
            "read": battle_state.total_bytes,
            "total_requests": battle_state.total_requests,
            "tbs": battle_state.total_bytes as f64 / 1024.0 / 1024.0 / duration,
            "non_success_requests": battle_state.status_counts.iter()
                .filter(|(&ref code, _)| code.parse::<i16>().unwrap_or(200) >= 400)
                .map(|(_, &count)| count)
                .sum::<u64>(),
            "errors": {
                "connect": 0, // We don't track these separately yet
                "read": 0,
                "write": 0,
                "timeout": 0
            },
            "latency": {
                "avg": battle_state.latency_p50.unwrap_or(0.0),
                "max": battle_state.latency_p99.unwrap_or(0.0)
            },
            "rps": {
                "avg": total_rps,
                "max": total_rps
            },
            // Enhanced verbose data
            "status_counts": battle_state.status_counts,
            "latency_p50": battle_state.latency_p50,
            "latency_p90": battle_state.latency_p90,
            "latency_p99": battle_state.latency_p99,
            "duration_secs": duration,
            "connections": battle_state.total_connections,
            "threads": battle_state.total_connections, // Using connections as threads for Cloudflare
            "method": battle_state.method,
            "url": battle_state.target,
            "battle_id": battle_state.battle_id,
            "is_running": battle_state.is_running,
            "active_bazookas": battle_state.active_bazookas,
            "completed_bazookas": battle_state.completed_bazookas,
        });

        Ok(report)
    }

    pub fn cleanup_battle(&mut self) {
        self.battle_state = None;
        self.next_bazooka_id = 1;
    }
}

// Durable Object implementation
#[durable_object]
pub struct WaspDurableObject {
    state: State,
    env: Env,
    wasp_state: WaspDurableState,
}

impl WaspDurableObject {
    async fn load_state(&self) -> Result<WaspDurableState> {
        match self.state.storage().get("wasp_state").await? {
            Some(state) => Ok(state),
            None => {
                // Initialize with default state
                let wasp_id = "CloudflareWasp".to_string();
                let port = "443".to_string();
                let hive_url = match self.env.var("HIVE_URL") {
                    Ok(var) => var.to_string(),
                    Err(_) => "http://localhost:4269".to_string(),
                };
                let bazooka_worker_url = match self.env.var("BAZOOKA_WORKER_URL") {
                    Ok(var) => var.to_string(),
                    Err(_) => "https://wwb-cf-bazooka.dragohm.workers.dev".to_string(),
                };
                let wasp_worker_url = match self.env.var("WASP_WORKER_URL") {
                    Ok(var) => var.to_string(),
                    Err(_) => "https://wwb-cf-wasp.dragohm.workers.dev".to_string(),
                };
                let hive_token = self.env.secret("HIVE_TOKEN").ok().map(|s| s.to_string());

                let wasp_state = WaspDurableState::new(
                    wasp_id,
                    port,
                    hive_url,
                    hive_token,
                    bazooka_worker_url,
                    wasp_worker_url,
                );
                
                // Save the initial state
                self.state.storage().put("wasp_state", &wasp_state).await?;
                Ok(wasp_state)
            }
        }
    }

   pub async fn save_state(&self, wasp_state: &WaspDurableState) -> Result<()> {
        self.state.storage().put("wasp_state", wasp_state).await?;
        Ok(())
    }
}

impl DurableObject for WaspDurableObject {
    fn new(state: State, env: Env) -> Self {
        Self {
            state,
            env,
            wasp_state: WaspDurableState::new(
                "CloudflareWasp".to_string(),
                "443".to_string(),
                "http://localhost:4269".to_string(),
                None,
                "http://localhost:8787".to_string(),
                "http://localhost:8989".to_string(),
            ),
        }
    }

    async fn fetch(&self, mut req: Request) -> Result<Response> {
        let url = req.url()?;
        let path = url.path();

        match path {
            "/fire" => {
                if req.method() != Method::Put {
                    return Response::error("Method not allowed", 405);
                }

                let fire_request: crate::types::FireRequest = req.json().await?;
                
                // Load current state
                let mut wasp_state = self.wasp_state.clone();
                wasp_state.hive_url = self.env.var("HIVE_URL").expect("HIVE_URL is not set").to_string();
                wasp_state.bazooka_worker_url = self.env.var("BAZOOKA_WORKER_URL").expect("BAZOOKA_WORKER_URL is not set").to_string();
                wasp_state.wasp_worker_url = self.env.var("WASP_WORKER_URL").expect("WASP_WORKER_URL is not set").to_string();
                wasp_state.wasp_id = Uuid::new_v4().to_string();
                wasp_state.initialize_battle(&fire_request)?;
                self.save_state(&wasp_state).await?;
                // Spawn initial bazookas
                let total_connections = fire_request.c.unwrap_or(50);

                let initial_bazookas =  total_connections;

                for _ in 0..initial_bazookas {
                    let bazooka_request = wasp_state.spawn_bazooka()?;
                    let bazooka_worker_url = self.env.var("BAZOOKA_WORKER_URL").expect("BAZOOKA_WORKER_URL is not set").to_string();
                    // Spawn bazooka worker (fire-and-forget)
                    let req_body = serde_json::to_string(&bazooka_request)?;
                    spawn_local(async move {
                        let mut req_init = RequestInit::new();
                        req_init.with_method(Method::Post);
                        req_init.with_body(Some(req_body.into()));
                        let _ = Fetch::Request(Request::new_with_init(
                            &format!("{}/start", bazooka_worker_url),
                            &req_init
                        ).unwrap()).send().await;
                    });
                }
             
                // Save the updated state
                self.save_state(&wasp_state).await?;

                let response = crate::types::FireResponse {
                    status: "200".to_string(),
                    message: "🚀 I'M A'FIRIN' MAH ROCKETS! (Cloudflare style)".to_string(),
                };

                Response::from_json(&response)
            }
            "/bazooka_completed" => {
                if req.method() != Method::Post {
                    return Response::error("Method not allowed", 405);
                }

                let command: BazookaCommand = req.json().await?;
                
                // Load current state
                let mut wasp_state = self.load_state().await?;
                if let Some(next_bazooka_request) = wasp_state.handle_bazooka_completed(command,&self.state).await? {
                    // Spawn next bazooka (fire-and-forget)
                    let bazooka_worker_url = wasp_state.bazooka_worker_url.clone();
                    let req_body = serde_json::to_string(&next_bazooka_request)?;
                    spawn_local(async move {
                        let mut req_init = RequestInit::new();
                        req_init.with_method(Method::Post);
                        req_init.with_body(Some(req_body.into()));
                        let _ = Fetch::Request(Request::new_with_init(
                            &format!("{}/start", bazooka_worker_url),
                            &req_init
                        ).unwrap()).send().await;
                    });
                }
                else {
                    
                }

                // // Save the updated state
                // self.save_state(&wasp_state).await?;

                Response::ok("Bazooka completed")
            }
            "/battlereport" => {
                if req.method() != Method::Get {
                    return Response::error("Method not allowed", 405);
                }

                let wasp_state = self.load_state().await?;
                let report = wasp_state.get_battle_report()?;
                Response::from_json(&report)
            }
            "/ceasefire" => {
                if req.method() != Method::Get {
                    return Response::error("Method not allowed", 405);
                }

                // Load current state
                let mut wasp_state = self.load_state().await?;
                if let Some(ref mut battle_state) = wasp_state.battle_state {
                    battle_state.is_running = false;
                }

                // Save the updated state
                self.save_state(&wasp_state).await?;

                let response = crate::types::FireResponse {
                    status: "200".to_string(),
                    message: "Ok i stops (Cloudflare style)".to_string(),
                };

                Response::from_json(&response)
            }
            "/die" => {
                if req.method() != Method::Delete {
                    return Response::error("Method not allowed", 405);
                }

                // Load current state
                let mut wasp_state = self.load_state().await?;
                wasp_state.heartbeat_active = false;
                wasp_state.cleanup_battle();

                // Save the updated state
                self.save_state(&wasp_state).await?;

                let response = serde_json::json!({
                    "status": "shutting_down",
                    "message": "Wasp is shutting down... Heartbeat stopped and state cleaned up.",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });

                Response::from_json(&response)
            }
            _ => {
                Response::error("Not found", 404)
            }
        }
    }

    async fn alarm(&self) -> Result<Response> {
        // Load current state
        let wasp_state = self.load_state().await?;
        
        // Handle heartbeat to hive
        if wasp_state.heartbeat_active {
            let hive_client = crate::hive_client::HiveClient::new(
                wasp_state.hive_url.clone(),
                wasp_state.hive_token.clone(),
            );

            if let Err(e) = hive_client.heartbeat(wasp_state.port.parse().unwrap_or(443)).await {
                console_log!("Heartbeat error: {:?}", e);
            }

            // Set next alarm in 30 seconds
            let next_alarm = std::time::Duration::from_secs(30);
            self.state.storage().set_alarm(next_alarm).await?;
        }

        Response::ok("Heartbeat sent")
    }
} 