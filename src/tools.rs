use std::fs;
use std::path::Path;
use std::process::Command as StdCommand;
use tokio::process::Command as TokioCommand;
use tokio::fs as TokioFs;

use crate::provider::OllamaProvider;

/// Security Tools - like OpenClaw tools but for security
pub struct SecurityTools {
    ai: OllamaProvider,
}

impl SecurityTools {
    pub fn new() -> Self {
        Self {
            ai: OllamaProvider::new(),
        }
    }
    
    /// Audit a local file
    pub async fn audit_file(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let content = TokioFs::read_to_string(path).await?;
        let truncated = &content.chars().take(5000).collect::<String>();
        
        let prompt = format!(
            "Analyze this code for security vulnerabilities:\n\nFile: {}\n\n{}\n\nIdentify:\n1. Critical vulnerabilities\n2. Logic flaws\n3. Suggested fixes",
            path, truncated
        );
        
        let system = "You are a security code auditor. Be concise and specific.";
        let result = self.ai.chat(&prompt, system).await?;
        
        Ok(format!("🔍 File Audit: {}\n\n{}", path, result))
    }
    
    /// Clone and audit a GitHub repository
    pub async fn audit_repo(
0026self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Create temp directory
        let temp_dir = std::env::temp_dir().join(format!("pi-audit-{}", uuid::Uuid::new_v4()));
        let temp_path = temp_dir.to_str().unwrap();
        
        println!("📦 Cloning {}...", url);
        
        let output = TokioCommand::new("git")
            .args([&"clone",
                "--depth",
                "1",
                url,
                temp_path,
            ])
            .output()
            .await?;
        
        if !output.status.success() {
            return Err("Failed to clone repository".into());
        }
        
        // Find code files
        let mut files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(temp_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let ext = path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");
                
                if matches!(ext, "rs" | "py" | "js" | "ts" | "sol") {
                    files.push(path);
                }
            }
        }
        
        let file_count = files.len();
        
        // Analyze sample file with AI
        let analysis = if let Some(sample) = files.first() {
            let content = fs::read_to_string(sample)?;
            let truncated = &content.chars().take(4000).collect::<String>();
            
            let prompt = format!(
                "Repository: {}\nFile: {}\nCode:\n{}\n\nSecurity analysis:",
                url,
                sample.file_name().unwrap().to_str().unwrap_or("unknown"),
                truncated
            );
            
            self.ai.chat(&prompt, "You are a security auditor analyzing a repository.").await?
        } else {
            "No code files found to analyze.".to_string()
        };
        
        // Cleanup
        let _ = std::fs::remove_dir_all(temp_path);
        
        Ok(format!(
            "📦 Repository: {}\n   Files found: {}\n   Analyzed: sample file\n\n🔍 Analysis:\n{}",
            url, file_count, analysis
        ))
    }
    
    /// Network scan target (requires nmap)
    pub async fn scan_target(&self, target: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("📡 Scanning {} with nmap...", target);
        
        let output = TokioCommand::new("nmap")
            .args([
                "-sV",
                "-p-",
                "--top-ports",
                "100",
                "--open",
                target,
            ])
            .output()
            .await
            .map_err(|_| "nmap not installed. Run: sudo apt install nmap")?;
        
        let scan_output = if output.status.success() {
            String::from_utf8_lossy(&output.stdout)
        } else {
            String::from_utf8_lossy(&output.stderr)
        };
        
        let truncated = &scan_output.chars().take(4000).collect::<String>();
        
        let prompt = format!(
            "Nmap scan results:\n{}\n\nAnalyze for security risks and vulnerable services.",
            truncated
        );
        
        let result = self.ai.chat(&prompt, "You are a network security analyst.").await?;
        
        Ok(format!("📡 Network Scan: {}\n\n{}", target, result))
    }
    
    /// Write security patch to file
    pub async fn write_patch(&self, path: &str, patch: &str) -> Result<String, Box<dyn std::error::Error>> {
        TokioFs::write(path, patch).await?;
        Ok(format!("✅ Patch written to {}", path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tools_creation() {
        let tools = SecurityTools::new();
        // Smoke test
        assert!(true);
    }
}
