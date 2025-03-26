use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ApiClient {
    async fn request(&self,endpoint:&str,payload: Option<Value>) -> Result<String,Box<dyn std::error::Error>>;
}
