use crate::provider::OllamaProvider;
use crate::tools::SecurityTools;
use regex::Regex;
use std::collections::HashMap;

/// Agent action types (Tool Calling pattern like OpenClaw)
#[derive(Debug, Clone)]
pub enum AgentAction {
    Think { thought: String },
    UseTool { name: String, args: HashMap<String, String> },
    Respond { message: String },
}

/// Security Agent - Core AI logic (OpenClaw architecture)
pub struct SecurityAgent {
    ai: OllamaProvider,
    tools: SecurityTools,
    history: Vec<(String, String)>, // (role, message)
}

impl SecurityAgent {
    pub fn new() -> Self {
        Self {
            ai: OllamaProvider::new(),
            tools: SecurityTools::new(),
            history: Vec::new(),
        }
    }
    
    /// Run autonomous task (think -> act -> observe loop)
    pub async fn run_task(&mut self, task: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("🧠 Planning task execution...");
        
        // Step 1: Plan using AI
        let plan_prompt = format!(
            "Task: {}\n\nCurrent directory: {}\n\nPlan your approach step by step. What tools will you use? \nAvailable tools: audit_file, audit_repo, scan_target, read_file, write_patch",
            task,
            std::env::current_dir()?.display()
        );
        
        let system = "You are Pi Swarm Security Agent. Plan tasks using available tools.";
        let plan = self.ai.chat(&plan_prompt, system).await?;
        
        println!("📝 Plan: {}\n", plan.lines().take(3).collect::<Vec<_>>().join("\n"));
        
        // Step 2: Execute based on plan content
        let result = self.execute_plan(&plan, task).await?;
        
        // Step 3: Summarize
        let summary_prompt = format!(
            "Task: {}\nExecution Result:\n{}\n\nProvide a brief summary of what was accomplished.",
            task, result
        );
        
        let summary = self.ai.chat(&summary_prompt, "Summarize the execution results.").await?;
        
        Ok(format!("✅ Task Completed\n\n📝 Summary:\n{}\n\n📊 Details:\n{}", summary, result))
    }
    
    /// Execute plan based on identified actions
    async fn execute_plan(&mut self,
        plan: &str,
        original_task: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        // Detect if task mentions file operations
        if plan.contains("audit") || original_task.contains("audit") {
            // Determine target from task
            let repo_regex = Regex::new(r"https?://[^\s]+")?;
            let file_regex = Regex::new(r"[\w/\-]+\.(rs|py|js|ts|sol)")?;
            
            // Check for repo URL
            if let Some(mat) = repo_regex.find(original_task) {
                results.push("🔍 Found repository to audit.".to_string());
                match self.tools.audit_repo(mat.as_str()).await {
                    Ok(r) => results.push(r),
                    Err(e) => results.push(format!("Error: {}", e)),
                }
            }
            // Check for file path
            else if let Some(mat) = file_regex.find(original_task) {
                results.push(format!("📄 Found file to audit: {}", mat.as_str()));
                match self.tools.audit_file(mat.as_str()).await {
                    Ok(r) => results.push(r),
                    Err(e) => results.push(format!("Error: {}", e)),
                }
            }
        }
        
        // Detect scan operations
        if plan.contains("scan") || original_task.contains("scan") {
            let ip_regex = Regex::new(r"(\d{1,3}\.){3}\d{1,3}")?;
            
            if let Some(mat) = ip_regex.find(original_task) {
                results.push(format!("📡 Found target to scan: {}", mat.as_str()));
                match self.tools.scan_target(mat.as_str()).await {
                    Ok(r) => results.push(r),
                    Err(e) => results.push(format!("Error: {}", e)),
                }
            }
        }
        
        // If no specific action detected, analyze locally
        if results.is_empty() {
            results.push("🔍 Analyzing current directory...".to_string());
            
            let current_dir = std::env::current_dir()?;
            let files: Vec<String> = std::fs::read_dir(current_dir)?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some())
                .map(|e| e.file_name().to_string_lossy().to_string())
                .filter(|f| f.ends_with(".rs") || f.ends_with(".py") || f.ends_with(".js"))
                .take(5)
                .collect();
            
            if !files.is_empty() {
                results.push(format!("Found code files: {:?}", files));
                
                // Analyze with AI
                let prompt = format!(
                    "In directory with files: {:?}\n\nTask: {}\n\nWhat security checks should be performed?",
                    files, original_task
                );
                
                let analysis = self.ai.chat(&prompt, "Security analyst.").await?;
                results.push(format!("🧠 Analysis:\n{}", analysis));
            } else {
                results.push("No code files found in current directory.".to_string());
            }
        }
        
        Ok(results.join("\n\n"))
    }
    
    /// Direct chat without tool execution
    pub async fn chat(&mut self,
        message: &str,
        system_prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.history.push(("user".to_string(), message.to_string()));
        
        // Build context from history
        let context = self.build_context(system_prompt);
        
        let response = self.ai.chat(message, &context).await?;
        
        self.history.push(("assistant".to_string(), response.clone()));
        
        // Limit history
        if self.history.len() > 20 {
            self.history.drain(0..2);
        }
        
        Ok(response)
    }
    
    fn build_context(&self, system_prompt: &str) -> String {
        let history_str = self.history.iter()
            .map(|(role, msg)| format!("{}: {}", role, msg.lines().next().unwrap_or("")))
            .collect::<Vec<_>>()
            .join("\n");
        
        if history_str.is_empty() {
            system_prompt.to_string()
        } else {
            format!("{}\n\nPrevious conversation:\n{}", system_prompt, history_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agent_creation() {
        let agent = SecurityAgent::new();
        assert!(agent.history.is_empty());
    }
}
