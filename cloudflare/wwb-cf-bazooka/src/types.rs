use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BazookaStartRequest {
    pub target: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout: u32,
    pub connections: u32,
    pub threads: u32,
    pub duration_secs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BazookaStats {
    pub requests: u64,
    pub bytes: u64,
    pub rps: f64,
    pub status_counts: HashMap<String, u64>,
    pub latency_p50: Option<f64>,
    pub latency_p90: Option<f64>,
    pub latency_p99: Option<f64>,
    pub duration_secs: u64,
    pub is_running: bool,
    pub error: Option<String>,
    pub bazookas_completed: u32,
    pub total_bazookas: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleRequest {
    pub id: u32, // This is the bazooka_id
    pub target: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout: u32,
    pub battle_id: String, // To identify which battle this bazooka belongs to
    pub battle_start_time: u64, // Unix timestamp when battle started
    pub battle_duration_secs: u32, // Total battle duration
    pub callback_url: String, // URL to call back to the wasp worker
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BazookaResult {
    pub bazooka_id: u32,
    pub battle_id: String,
    pub requests_completed: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_bytes: u64,
    pub total_latency_ms: f64,
    pub status_counts: HashMap<String, u64>,
    pub latency_distribution: Vec<f64>,
    pub rps: f64,
    pub latency_p50: Option<f64>,
    pub latency_p90: Option<f64>,
    pub latency_p99: Option<f64>,
    pub duration_secs: f64,
    pub loops_completed: u32, // Number of 500-request loops completed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestResult {
    pub status_code: u16,
    pub latency_ms: f64,
    pub bytes: u64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BazookaCommand {
    pub command: String, // "completed" or "spawn_next"
    pub bazooka_id: u32,
    pub result: Option<BazookaResult>,
}