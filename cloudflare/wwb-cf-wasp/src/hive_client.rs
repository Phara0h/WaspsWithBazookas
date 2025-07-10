use worker::*;
use crate::types::HiveCheckinResponse;

#[derive(Debug, Clone)]
pub struct HiveClient {
    hive_url: String,
    hive_token: Option<String>,
}

impl HiveClient {
    pub fn new(hive_url: String, hive_token: Option<String>) -> Self {
        Self { hive_url, hive_token }
    }

    pub async fn checkin(&self, port: u16) -> Result<HiveCheckinResponse> {
        let url = format!("{}/wasp/checkin/{}", self.hive_url, port);
        let mut req_init = RequestInit::new();
        req_init.with_method(Method::Get);
        
        if let Some(ref token) = self.hive_token {
            let headers = Headers::new();
            headers.set("wwb-token", token)?;
            req_init.with_headers(headers);
        }
        
        let mut response = Fetch::Request(Request::new_with_init(&url, &req_init)?).send().await?;
        
        if response.status_code() == 200 {
            let result: HiveCheckinResponse = response.json().await?;
            Ok(result)
        } else {
            let error_text = response.text().await?;
            Err(Error::RustError(format!("Hive checkin error: {} - {}", response.status_code(), error_text)))
        }
    }

    pub async fn heartbeat(&self, port: u16) -> Result<()> {
        let url = format!("{}/wasp/heartbeat/{}", self.hive_url, port);
        let mut req_init = RequestInit::new();
        req_init.with_method(Method::Get);
        
        if let Some(ref token) = self.hive_token {
            let headers = Headers::new();
            headers.set("wwb-token", token)?;
            req_init.with_headers(headers);
        }
        
        let mut response = Fetch::Request(Request::new_with_init(&url, &req_init)?).send().await?;
        
        if response.status_code() == 200 {
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(Error::RustError(format!("Hive heartbeat error: {} - {}", response.status_code(), error_text)))
        }
    }

    pub async fn report_battle_result(&self, wasp_id: &str, battle_stats: serde_json::Value) -> Result<()> {
        let url = format!("{}/wasp/reportin/{}", self.hive_url, wasp_id);
        let mut req_init = RequestInit::new();
        req_init.with_method(Method::Put);
        req_init.with_body(Some(battle_stats.to_string().into()));
        
        if let Some(ref token) = self.hive_token {
            let headers = Headers::new();
            headers.set("wwb-token", token)?;
            req_init.with_headers(headers);
        }
        
        let mut response = Fetch::Request(Request::new_with_init(&url, &req_init)?).send().await?;
        
        if response.status_code() == 200 {
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(Error::RustError(format!("Hive report error: {} - {}", response.status_code(), error_text)))
        }
    }
} 