use worker::*;
use crate::types::{BazookaRequest, BazookaResult};

#[derive(Debug, Clone)]
pub struct BazookaClient {
    bazooka_url: String,
}

impl BazookaClient {
    pub fn new(bazooka_url: String) -> Self {
        Self { bazooka_url }
    }

    pub async fn execute(&self, bazooka_request: BazookaRequest) -> Result<BazookaResult> {
        let mut req_init = RequestInit::new();
        req_init.with_method(Method::Post);
        req_init.with_body(Some(serde_json::to_string(&bazooka_request)?.into()));
        
        let mut response = Fetch::Request(Request::new_with_init(&format!("{}/start", self.bazooka_url), &req_init)?).send().await?;
        
        if response.status_code() == 200 {
            let result: BazookaResult = response.json().await?;
            Ok(result)
        } else {
            let error_text = response.text().await?;
            Err(Error::RustError(format!("Bazooka worker error: {}", error_text)))
        }
    }
} 