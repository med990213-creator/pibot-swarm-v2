use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::agent::SecurityAgent;
use crate::tools::SecurityTools;

/// Telegram Gateway Integration - OpenClaw-style channel
pub struct TelegramGateway {
    token: String,
    agent: Arc<Mutex<SecurityAgent>>,
    tools: SecurityTools,
}

impl TelegramGateway {
    pub fn new() -> Result<Self, String> {
        let token = env::var("PI_TELEGRAM_TOKEN")
            .map_err(|_| "Set PI_TELEGRAM_TOKEN environment variable")?;
        
        Ok(Self {
            token,
            agent: Arc::new(Mutex::new(SecurityAgent::new())),
            tools: SecurityTools::new(),
        })
    }
    
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🛡️ Telegram Gateway starting...");
        println!("   Bot token: {}...", &self.token[..20.min(self.token.len())]);
        println!();
        println!("Send /start to your bot on Telegram to begin");
        
        // In real implementation, this would start the webhook/polling loop
        // For now, simulate interaction
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
    
    /// Handle incoming Telegram command
    async fn handle_command(&self,
        chat_id: i64,
        command: &str,
        args: &str,
    ) -> String {
        match command {
            "/start" => self.cmd_start(),
            "/status" => self.cmd_status().await,
            "/audit" => self.cmd_audit(args).await,
            "/scan" => self.cmd_scan(args).await,
            "/task" => self.cmd_task(args).await,
            "/agent" => self.cmd_agent(args).await,
            "/help" => self.cmd_help(),
            _ => "Unknown command. Use /help".to_string(),
        }
    }
    
    fn cmd_start(&self) -> String {
        "🛡️ Pi Swarm Security Edition (Rust)\n\nCommands:\n\
        /status - Check system\n\
        /audit <target> - Security audit\n\
        /scan <ip> - Network scan\n\
        /task <desc> - Autonomous task\n\
        /agent <msg> - Chat with AI\n\
        /help - Show help".to_string()
    }
    
    async fn cmd_status(&self) -> String {
        use crate::provider::check_ollama;
        
        let ollama = match check_ollama().await {
            Ok(_) => "🟢 Connected",
            Err(_) => "🔴 Disconnected",
        };
        
        format!("🛡️ Pi Swarm Status\nAI Brain: {}\nSecurity Tools: 🟢 Ready", ollama)
    }
    
    async fn cmd_audit(&self, target: &str) -> String {
        if target.is_empty() {
            return "Usage: /audit <file_or_url>".to_string();
        }
        
        match self.tools.audit_repo(target).await {
            Ok(result) => {
                // Truncate for Telegram
                if result.len() > 4000 {
                    format!("{}\n\n[...truncated]", &result[..4000])
                } else {
                    result
                }
            }
            Err(e) => format!("❌ Audit failed: {}", e),
        }
    }
    
    async fn cmd_scan(&self, target: &str) -> String {
        if target.is_empty() {
            return "Usage: /scan <ip_or_range>".to_string();
        }
        
        match self.tools.scan_target(target).await {
            Ok(result) => {
                if result.len() > 4000 {
                    format!("{}\n\n[...truncated]", &result[..4000])
                } else {
                    result
                }
            }
            Err(e) => format!("❌ Scan failed: {}", e),
        }
    }
    
    async fn cmd_task(&self, desc: &str) -> String {
        if desc.is_empty() {
            return "Usage: /task <security_task_description>".to_string();
        }
        
        let mut agent = self.agent.lock().await;
        match agent.run_task(desc).await {
            Ok(result) => {
                if result.len() > 4000 {
                    format!("{}\n\n[...truncated]", &result[..4000])
                } else {
                    result
                }
            }
            Err(e) => format!("❌ Task failed: {}", e),
        }
    }
    
    async fn cmd_agent(&self, message: &str) -> String {
        if message.is_empty() {
            return "Usage: /agent <message>".to_string();
        }
        
        let mut agent = self.agent.lock().await;
        let system = "You are Pi Swarm Security Agent. Answer concisely.";
        
        match agent.chat(message, system).await {
            Ok(response) => {
                if response.len() > 4000 {
                    format!("{}\n\n[...truncated]", &response[..4000])
                } else {
                    response
                }
            }
            Err(e) => format!("❌ Error: {}", e),
        }
    }
    
    fn cmd_help(&self) -> String {
        "🛡️ Pi Swarm Security (Rust)\n\n\
        /status - System status\n\
        /audit <target> - Audit file/repo\n\
        /scan <ip> - Network scan\n\
        /task <desc> - Autonomous task\n\
        /agent <msg> - AI chat\n\
        /help - This message".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cmd_help() {
        env::set_var("PI_TELEGRAM_TOKEN", "test_token_fake");
        let gateway = TelegramGateway::new().unwrap();
        let help = gateway.cmd_help();
        assert!(help.contains("/status"));
        assert!(help.contains("/audit"));
    }
}
