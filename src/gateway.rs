use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::session::SessionManager;

/// Pi Swarm Gateway - OpenClaw-style daemon
pub struct PiGateway {
    port: u16,
    sessions: Arc<Mutex<SessionManager>>,
}

impl PiGateway {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            sessions: Arc::new(Mutex::new(SessionManager::new())),
        }
    }
    
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr).await?;
        
        println!("🛡️ Gateway listening on http://{}", addr);
        println!("   Sessions: active");
        println!();
        
        loop {
            let (socket, addr) = listener.accept().await?;
            let sessions = Arc::clone(&self.sessions);
            
            tokio::spawn(async move {
                println!("📡 New connection from {}", addr);
                handle_connection(socket, sessions).await;
            });
        }
    }
}

async fn handle_connection(
    _socket: tokio::net::TcpStream,
    _sessions: Arc<Mutex<SessionManager>>,
) {
    // Handle incoming requests (HTTP or custom protocol)
    // For now, just accept the connection
    println!("   Connection handled");
}

impl Clone for PiGateway {
    fn clone(&self) -> Self {
        Self {
            port: self.port,
            sessions: Arc::clone(&self.sessions),
        }
    }
}
