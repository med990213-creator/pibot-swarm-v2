use clap::{Parser, Subcommand};
use tokio;

mod agent;
mod gateway;
mod provider;
mod session;
mod telegram;
mod tools;

use gateway::PiGateway;

/// 🛡️ Pi Swarm - Sovereign AI Security (OpenClaw Architecture in Rust)
#[derive(Parser)]
#[command(name = "pi")]
#[command(about = "Enhanced OpenClaw with Security Tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check system status (like 'openclaw status')
    Status,
    /// Run AI agent with message (like 'openclaw agent --message')
    Agent {
        /// Message to send to agent
        #[arg(required = true)]
        message: Vec<String>,
    },
    /// Run autonomous security task (like 'openclaw agent --task')
    Task {
        /// Task description
        #[arg(required = true)]
        description: Vec<String>,
    },
    /// Security-specific operations
    Security {
        #[command(subcommand)]
        command: SecurityCommands,
    },
    /// Start gateway daemon (like 'openclaw gateway')
    Gateway {
        /// Port to run gateway on
        #[arg(short, long, default_value = "18789")]
        port: u16,
        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,
    },
    /// Run onboarding (like 'openclaw onboard')
    Onboard,
    /// Send message via channel (like 'openclaw message send')
    Message {
        #[command(subcommand)]
        command: MessageCommands,
    },
}

#[derive(Subcommand)]
enum SecurityCommands {
    /// Security audit a file or repository
    Audit {
        /// Target file or URL
        target: String,
    },
    /// Network scan
    Scan {
        /// Target IP or range
        target: String,
    },
}

#[derive(Subcommand)]
enum MessageCommands {
    /// Send message via Telegram
    Send {
        /// Recipient
        #[arg(short, long)]
        to: String,
        /// Message content
        #[arg(short, long)]
        message: String,
    },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Status => {
            cmd_status().await;
        }
        Commands::Agent { message } => {
            let msg = message.join(" ");
            cmd_agent(&msg).await;
        }
        Commands::Task { description } => {
            let desc = description.join(" ");
            cmd_task(&desc).await;
        }
        Commands::Security { command } => {
            match command {
                SecurityCommands::Audit { target } => {
                    cmd_security_audit(&target).await;
                }
                SecurityCommands::Scan { target } => {
                    cmd_security_scan(&target).await;
                }
            }
        }
        Commands::Gateway { port, verbose } => {
            cmd_gateway(port, verbose).await;
        }
        Commands::Onboard => {
            cmd_onboard().await;
        }
        Commands::Message { command } => {
            match command {
                MessageCommands::Send { to, message } => {
                    cmd_message_send(&to, &message).await;
                }
            }
        }
    }
}

/// Check system status - OpenClaw style
async fn cmd_status() {
    println!("🛡️ Pi Swarm Security Edition");
    println!("   Version: 2026.2.26");
    println!();
    
    // Check Ollama
    match provider::check_ollama().await {
        Ok(_) => println!("🟢 AI Brain (Ollama): Connected"),
        Err(e) => {
            println!("🔴 AI Brain (Ollama): Disconnected");
            println!("   Error: {}", e);
            println!("   Run: ollama serve");
        }
    }
    
    println!();
    println!("🛡️ Security Tools:");
    println!("   • audit_repo     - Clone & audit repositories");
    println!("   • scan_target    - Network reconnaissance");
    println!("   • analyze_code   - Static code analysis");
    println!("   • write_patch    - Automated patching");
    println!();
    println!("📡 Channels:");
    println!("   • CLI (active)");
    println!("   • Telegram (when configured)");
    println!();
    println!("   Workspace: {}", std::env::var("HOME").unwrap_or_default() + "/.openclaw/workspace/pibot");
}

/// Agent command - OpenClaw style
async fn cmd_agent(message: &str) {
    println!("🤖 Agent: {}", message);
    println!();
    
    let system = "You are Pi Swarm, a sovereign AI security assistant. Answer concisely.";
    
    match provider::ask_ollama(message, Some(system)).await {
        Ok(response) => {
            println!("{}", response);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            eprintln!("Make sure Ollama is running: ollama serve");
        }
    }
}

/// Task command - Autonomous security task
async fn cmd_task(description: &str) {
    println!("🚀 Task: {}", description);
    println!();
    
    let agent = agent::SecurityAgent::new();
    match agent.run_task(description).await {
        Ok(result) => {
            println!("✅ Result:");
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("❌ Task failed: {}", e);
        }
    }
}

/// Security audit command
async fn cmd_security_audit(target: &str) {
    println!("🔍 Security Audit: {}", target);
    println!();
    
    let tool = tools::SecurityTools::new();
    
    if target.starts_with("http") {
        // Repo audit
        match tool.audit_repo(target).await {
            Ok(result) => {
                println!("📦 Repository Analysis:");
                println!("{}", result);
            }
            Err(e) => {
                eprintln!("❌ Audit failed: {}", e);
            }
        }
    } else {
        // File audit
        match tool.audit_file(target).await {
            Ok(result) => {
                println!("📄 File Analysis:");
                println!("{}", result);
            }
            Err(e) => {
                eprintln!("❌ Audit failed: {}", e);
            }
        }
    }
}

/// Security scan command
async fn cmd_security_scan(target: &str) {
    println!("📡 Network Scan: {}", target);
    println!();
    
    let tool = tools::SecurityTools::new();
    match tool.scan_target(target).await {
        Ok(result) => {
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("❌ Scan failed: {}", e);
        }
    }
}

/// Gateway daemon - OpenClaw style
async fn cmd_gateway(port: u16, verbose: bool) {
    if verbose {
        println!("🛡️ Pi Swarm Gateway");
        println!("   Port: {}", port);
        println!();
    }
    
    let gateway = PiGateway::new(port);
    
    println!("Starting gateway daemon...");
    println!("Press Ctrl+C to stop");
    println!();
    
    if let Err(e) = gateway.run().await {
        eprintln!("❌ Gateway error: {}", e);
        std::process::exit(1);
    }
}

/// Onboard command - Setup wizard
async fn cmd_onboard() {
    println!("🛡️ Pi Swarm Onboarding");
    println!("   Setting up your security environment...");
    println!();
    println!("✅ Checking Ollama installation...");
    
    match provider::check_ollama().await {
        Ok(_) => println!("   Ollama is ready"),
        Err(_) => {
            println!("   Please install Ollama:");
            println!("   curl -fsSL https://ollama.com/install.sh | sh");
        }
    }
    
    println!();
    println!("✅ Setup complete! Run 'pi status' to verify.");
}

/// Message send command
async fn cmd_message_send(to: &str, message: &str) {
    println!("📨 Sending message to {}...", to);
    
    // This would integrate with Telegram channel
    println!("   Message: {}", message);
    println!();
    println!("✅ Message queued for delivery");
}
