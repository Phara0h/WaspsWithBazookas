use serde::{Deserialize, Serialize, Deserializer};
use std::collections::HashMap;

fn de_opt_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let val: Option<serde_json::Value> = Option::deserialize(deserializer)?;
    match val {
        Some(serde_json::Value::Number(n)) => n.as_u64().map(|v| v as u32).ok_or_else(|| Error::custom("Invalid number")).map(Some),
        Some(serde_json::Value::String(s)) => s.parse::<u32>().map(Some).map_err(|_| Error::custom("Invalid string for u32")),
        Some(_) => Err(Error::custom("Invalid type for u32")),
        None => Ok(None),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireRequest {
    pub target: String,
    #[serde(default, deserialize_with = "de_opt_u32")]
    pub t: Option<u32>, // threads
    #[serde(default, deserialize_with = "de_opt_u32")]
    pub c: Option<u32>, // connections
    #[serde(default, deserialize_with = "de_opt_u32")]
    pub d: Option<u32>, // duration
    #[serde(default, deserialize_with = "de_opt_u32")]
    pub timeout: Option<u32>,
    pub method: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurableObjectCommand {
    pub command: String,
    pub data: serde_json::Value,
}

// Battle state managed by Wasp Durable Object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleState {
    pub battle_id: String,
    pub start_time: u64, // Unix timestamp in milliseconds
    pub duration_secs: u32,
    pub target: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout: u32,
    pub total_connections: u32, // Total 'c' value
    pub active_bazookas: u32,
    pub completed_bazookas: u32,
    pub total_requests: u64,
    pub total_bytes: u64,
    pub total_rps: f64,
    pub status_counts: HashMap<String, u64>,
    pub latency_p50: Option<f64>,
    pub latency_p90: Option<f64>,
    pub latency_p99: Option<f64>,
    pub is_running: bool,
}

// Request sent to Bazooka worker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BazookaRequest {
    pub id: u32,
    pub target: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout: u32,
    pub battle_id: String,
    pub battle_start_time: u64,
    pub battle_duration_secs: u32,
    pub callback_url: String,
}

// Result returned by Bazooka worker
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

// Command from Bazooka to Wasp Durable Object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BazookaCommand {
    pub command: String, // "completed" or "spawn_next"
    pub bazooka_id: u32,
    pub result: Option<BazookaResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiveCheckinResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaspState {
    pub wasp_id: String,
    pub port: String,
    pub hive_url: String,
    pub hive_token: Option<String>,
    pub bazooka_worker_url: String,
    pub heartbeat_active: bool,
    pub battle_state: Option<BattleState>,
} 