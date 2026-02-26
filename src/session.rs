use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Session state (like OpenClaw sessions)
pub struct Session {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub state: SessionState,
}

#[derive(Debug)]
pub enum SessionState {
    Active,
    Idle,
    Terminated,
}

impl Session {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: now,
            last_activity: now,
            state: SessionState::Active,
        }
    }
    
    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
        self.state = SessionState::Active;
    }
    
    pub fn idle_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.last_activity).num_minutes()
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages active sessions (OpenClaw-style)
pub struct SessionManager {
    sessions: HashMap<String, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
    
    pub fn create_session(&mut self) -> String {
        let session = Session::new();
        let id = session.id.clone();
        self.sessions.insert(id.clone(), session);
        id
    }
    
    pub fn get_session(&self, id: &str) -> Option<&Session> {
        self.sessions.get(id)
    }
    
    pub fn get_session_mut(&mut self, id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(id)
    }
    
    pub fn terminate_session(&mut self, id: &str) {
        if let Some(session) = self.sessions.get_mut(id) {
            session.state = SessionState::Terminated;
        }
        self.sessions.remove(id);
    }
    
    pub fn list_sessions(&self) -> Vec<&Session> {
        self.sessions.values().collect()
    }
    
    pub fn cleanup_idle(&mut self, max_idle_minutes: i64) {
        let to_remove: Vec<String> = self
            .sessions
            .iter()
            .filter(|(_, s)| s.idle_minutes() > max_idle_minutes)
            .map(|(id, _)| id.clone())
            .collect();
        
        for id in to_remove {
            self.sessions.remove(&id);
        }
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_creation() {
        let session = Session::new();
        assert!(!session.id.is_empty());
        assert!(matches!(session.state, SessionState::Active));
    }
    
    #[test]
    fn test_session_manager() {
        let mut manager = SessionManager::new();
        let id = manager.create_session();
        assert!(manager.get_session(&id).is_some());
        manager.terminate_session(&id);
        assert!(manager.get_session(&id).is_none());
    }
}
