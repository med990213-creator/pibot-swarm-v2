use reqwest::Client;
use serde::{Deserialize, Serialize};

const OLLAMA_URL: &str = "http://localhost:11434/api/generate";
const MODEL: &str = "qwen2.5:1.5b";

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

/// Check if Ollama is running
pub async fn check_ollama() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .get("http://localhost:11434")
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await?;
    
    if res.status().is_success() {
        Ok(())
    } else {
        Err(format!("Ollama returned status: {}", res.status()).into())
    }
}

/// Ask Ollama AI for response
pub async fn ask_ollama(prompt: &str, system: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let req = OllamaRequest {
        model: MODEL.to_string(),
        prompt: prompt.to_string(),
        stream: false,
        system: system.map(String::from),
    };
    
    let res = client
        .post(OLLAMA_URL)
        .json(&req)
        .timeout(std::time::Duration::from_secs(120))
        .send()
        .await?;
    
    if !res.status().is_success() {
        return Err(format!("Ollama error: {}", res.status()).into());
    }
    
    let body: OllamaResponse = res.json().await?;
    Ok(body.response)
}

/// Ollama provider struct for agent use
pub struct OllamaProvider;

impl OllamaProvider {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn chat(&self, message: &str, context: &str) -> Result<String, Box<dyn std::error::Error>> {
        let system = if context.is_empty() {
            None
        } else {
            Some(context)
        };
        ask_ollama(message, system).await
    }
}
