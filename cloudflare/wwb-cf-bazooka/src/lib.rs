use worker::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod types;

use types::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    
    let url = req.url()?;
    let path = url.path();
    let method = req.method();
  
    let response = match (method, path) {
        (Method::Post, "/start") => handle_execute(req, &env, _ctx).await,
        _ => {
            let mut response = Response::error("Not Found", 404)?;
            response.headers_mut().set("Content-Type", "application/json")?;
            return Ok(response);
        }
    }?;
  
    Ok(response)
}

async fn handle_execute(mut req: Request, _env: &Env, ctx: Context) -> Result<Response> {
    // Parse the chunk request
    let fire_request: BattleRequest = req.json().await
        .map_err(|e| Error::RustError(format!("Failed to parse chunk request: {}", e)))?;
    
    console_log!("🎯 Bazooka {} executing for battle {}", 
                fire_request.id, fire_request.battle_id);
    
    // Clone fire_request for the background task
    let fire_request_clone = fire_request.clone();
    
    // Start the background work using ctx.waitUntil()
    ctx.wait_until(async move {
        // Execute this Bazooka (make 500 requests in loops until duration expires)
        let bazooka_result = match execute_bazooka(fire_request_clone.clone()).await {
            Ok(result) => result,
            Err(e) => {
                console_log!("🔴 Bazooka {}: Execution failed: {}", fire_request_clone.id, e);
                return;
            }
        };
    
        // Create BazookaCommand for callback
        let mut command = "completed".to_string();
        if bazooka_result.loops_completed == 500 {
            command = "spawn_next".to_string();
        }
    
        let bazooka_command = BazookaCommand {
            command: command,
            bazooka_id: bazooka_result.bazooka_id,
            result: Some(bazooka_result.clone()),
        };
    
        // Make the callback to the wasp worker
        let callback_result = make_callback_to_wasp(&fire_request.callback_url, &bazooka_command).await;
        match callback_result {
            Ok(_) => {
                console_log!("✅ Bazooka {}: Successfully called back to wasp worker", fire_request.id);
            }
            Err(e) => {
                console_log!("🔴 Bazooka {}: Failed to call back to wasp worker: {}", fire_request.id, e);
            }
        }
    });

    // Return immediate response
    let immediate_response = BazookaCommand {
        command: "started".to_string(),
        bazooka_id: fire_request.id,
        result: None,
    };
    
    let json = serde_json::to_string(&immediate_response)
        .map_err(|e| Error::RustError(format!("Failed to serialize response: {}", e)))?;
    
    let mut response = Response::ok(json)?;
    response.headers_mut().set("Content-Type", "application/json")?;
    Ok(response)
}

async fn execute_bazooka(fire_request: BattleRequest) -> Result<BazookaResult> {
    let start_time = worker::Date::now().as_millis();
    let mut all_results = Vec::new();
    let mut status_counts = HashMap::new();
    let mut loops_completed = 0;
  
    
    console_log!("🎯 Bazooka {}: Starting execution with 50 requests per loop", fire_request.id);
    for i in 0..200 {
    // Execute loops of 500 requests until battle duration expires
  
        // Check if battle duration has expired
        let current_time = worker::Date::now().as_millis() as f64;
        let elapsed_battle_time_secs = (current_time - start_time as f64) / 1000.0;
        
        if elapsed_battle_time_secs >= fire_request.battle_duration_secs as f64 {
            console_log!("⏰ Bazooka {}: Battle duration expired ({:.1}s), stopping execution", 
                        fire_request.id, elapsed_battle_time_secs);
            break;
        }
        
        // Check if we've exceeded 2000ms timeout for the entire bazooka execution
        let current_time = worker::Date::now().as_millis();
        let elapsed_time = current_time - start_time;
        if elapsed_time > 2000 {
            console_log!("⏰ Bazooka {}: Bazooka timeout exceeded ({}ms), stopping execution", 
                        fire_request.id, elapsed_time);
            break;
        }

        // Calculate remaining time for this request
        let remaining_time = 2000 - elapsed_time;
        if remaining_time <= 0 {
            console_log!("⏰ Bazooka {}: No time remaining for more requests, stopping", fire_request.id);
            break;
        }

        let config = fire_request.clone();
        let req = make_request_with_timeout(config, remaining_time as u32);
        match req.await {
            Ok(result) => {
                all_results.push(result.clone());
                *status_counts.entry(result.status_code.to_string()).or_insert(0) += 1;
            }
            Err(e) => {
                console_log!("🔴 Error making request: {}", e);
                all_results.push(RequestResult {
                    status_code: 0,
                    latency_ms: 0.0,
                    bytes: 0,
                    error: Some(e.to_string()),
                });
                *status_counts.entry("0".to_string()).or_insert(0) += 1;
            }
        }
          
        loops_completed += 1;
        
    }
    
    let end_time = worker::Date::now().as_millis();
    let duration_secs = (end_time - start_time) as f64 / 1000.0;
    
    // Calculate statistics
    let successful_requests = all_results.iter().filter(|r| r.error.is_none()).count() as u64;
    let failed_requests = all_results.len() as u64 - successful_requests;
    let total_bytes: u64 = all_results.iter().map(|r| r.bytes).sum();
    let total_latency_ms: f64 = all_results.iter().map(|r| r.latency_ms).sum();
    
    // Calculate latency percentiles
    let mut latencies: Vec<f64> = all_results.iter()
        .filter_map(|r| if r.error.is_none() { Some(r.latency_ms) } else { None })
        .collect();
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let latency_p50 = if !latencies.is_empty() {
        latencies.get(latencies.len() / 2).copied()
    } else { None };
    
    let latency_p90 = if !latencies.is_empty() {
        latencies.get((latencies.len() * 90 / 100).min(latencies.len().saturating_sub(1))).copied()
    } else { None };
    
    let latency_p99 = if !latencies.is_empty() {
        latencies.get((latencies.len() * 99 / 100).min(latencies.len().saturating_sub(1))).copied()
    } else { None };
    
   let rps = if duration_secs > 0.0 {
        successful_requests as f64 / duration_secs
    } else { 0.0 };
    
    console_log!("✅ Bazooka {} completed: {} requests in {} loops, {:.2} RPS, {:.1}s duration", 
                fire_request.id, successful_requests, loops_completed, rps, duration_secs);
    
    Ok(BazookaResult {
        bazooka_id: fire_request.id,
        battle_id: fire_request.battle_id,
        requests_completed: successful_requests,
        successful_requests,
        failed_requests,
        total_bytes,
        total_latency_ms,
        status_counts,
        latency_distribution: latencies,
        rps,
        latency_p50,
        latency_p90,
        latency_p99,
        duration_secs,
        loops_completed,
    })
}

async fn execute_request_loop(config: &BattleRequest, requests_per_loop: u32) -> Result<Vec<RequestResult>> {
    let mut results = Vec::new();
    let batch_size = 5; // Process requests in batches of 10
    
  //  for batch_start in (0..requests_per_loop).step_by(batch_size as usize) {
        // let batch_end = (batch_start + batch_size).min(requests_per_loop);
        // let mut batch_futures = Vec::new();
        
        // for _ in batch_start..batch_end {
        //     let config = config.clone();
        //     batch_futures.push(make_request(config));
        // }
        
        // // Wait for all requests in this batch to complete
        // for future in batch_futures {
        //     match future.await {
        //         Ok(result) => {
        //             results.push(result);
        //         }
        //         Err(e) => {
        //             console_log!("🔴 Error making request: {}", e);
        //             results.push(RequestResult {
        //                 status_code: 0,
        //                 latency_ms: 0.0,
        //                 bytes: 0,
        //                 error: Some(e.to_string()),
        //             });
        //         }
        //     }
        // }
        let config = config.clone();
        let req = make_request(config);
        match req.await {
            Ok(result) => {
                results.push(result);
            }
            Err(e) => {
                console_log!("🔴 Error making request: {}", e);
                results.push(RequestResult {
                    status_code: 0,
                    latency_ms: 0.0,
                    bytes: 0,
                    error: Some(e.to_string()),
                });
            }
        }
      
        // Small delay between batches - in Cloudflare Workers, we just continue
        // The event loop will naturally yield control
  //  }
    
    Ok(results)
}

async fn make_request_with_timeout(config: BattleRequest, timeout_ms: u32) -> Result<RequestResult> {
    let start_time = worker::Date::now().as_millis();
    
    // Build the request
    let mut request_init = RequestInit::new();
    request_init.with_method(method_from_string(&config.method)?);
    request_init.with_headers(Headers::new());
    request_init.with_body(Some(config.body.clone().into()));
    request_init.with_cf_properties(CfProperties {
        cache_ttl: Some(0),
        apps: Some(false),
        scrape_shield: Some(false),
        mirage: Some(false),
        minify: Some(MinifyConfig {
            js: false,
            html: false,
            css: false,
        }),
        polish: Some(PolishConfig::Off),
        resolve_override: Some(String::from("")),
        cache_key: Some(String::from("")),
        cache_ttl_by_status: Some(HashMap::new()),
        cache_everything: Some(false),
        ..Default::default()
    });
    
    let request = Request::new_with_init(&config.target, &request_init)?;

    // Make the request and check timeout after completion
    match Fetch::Request(request).send().await {
        Ok(mut response) => {
            let end_time = worker::Date::now().as_millis();
            let latency_ms = (end_time - start_time) as f64;
            
            // Check if we've exceeded the timeout
            if latency_ms > timeout_ms as f64 {
                return Ok(RequestResult {
                    status_code: 0,
                    latency_ms,
                    bytes: 0,
                    error: Some(format!("Request timeout after {}ms", timeout_ms)),
                });
            }
            
            let status_code = response.status_code();
            let bytes = response.bytes().await?.len() as u64;
            
            Ok(RequestResult {
                status_code,
                latency_ms,
                bytes,
                error: None,
            })
        }
        Err(e) => {
            let end_time = worker::Date::now().as_millis();
            let latency_ms = (end_time - start_time) as f64;
            
            Ok(RequestResult {
                status_code: 0,
                latency_ms,
                bytes: 0,
                error: Some(e.to_string()),
            })
        }
    }
}

async fn make_request(config: BattleRequest) -> Result<RequestResult> {
    make_request_with_timeout(config, 2000).await
}


fn method_from_string(method: &str) -> Result<Method> {
    match method.to_uppercase().as_str() {
        "GET" => Ok(Method::Get),
        "POST" => Ok(Method::Post),
        "PUT" => Ok(Method::Put),
        "DELETE" => Ok(Method::Delete),
        "PATCH" => Ok(Method::Patch),
        "HEAD" => Ok(Method::Head),
        "OPTIONS" => Ok(Method::Options),
        _ => Ok(Method::Get),
    }
}

async fn make_callback_to_wasp(callback_url: &str, data: &BazookaCommand) -> Result<()> {
    let json = serde_json::to_string(data)
        .map_err(|e| Error::RustError(format!("Failed to serialize callback data: {}", e)))?;
    
    let mut request_init = RequestInit::new();
    request_init.with_method(Method::Post);
    
    // Set headers
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("User-Agent", "WaspsWithBazookas-Bazooka/2.0")?;
    request_init.with_headers(headers);
    
    // Set body
    request_init.with_body(Some(json.into()));
    
    let request = Request::new_with_init(callback_url, &request_init)?;
    
    console_log!("📡 Bazooka: Making callback to wasp worker at {}", callback_url);
    
    match Fetch::Request(request).send().await {
        Ok(mut response) => {
            let status = response.status_code();
            let body = response.text().await?;
            if status >= 200 && status < 300 {
                console_log!("✅ Bazooka: Callback successful (status: {})", status);
                Ok(())
            } else {
                let error_msg = format!("Callback failed with status: {} and response: {}", status, body);
                console_log!("🔴 Bazooka: {}", error_msg);
                Err(Error::RustError(error_msg))
            }
        }
        Err(e) => {
            let error_msg = format!("Callback request failed: {}", e);
            console_log!("🔴 Bazooka: {}", error_msg);
            Err(Error::RustError(error_msg))
        }
    }
}