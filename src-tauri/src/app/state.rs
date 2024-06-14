use std::sync::Arc;

use anyhow::Result;
use tokio::sync::{Mutex, RwLock};

use crate::ssh::{Session, SessionInfo};

#[derive(Default)]
pub struct AppState {
    sessions: Mutex<Vec<Arc<RwLock<Session>>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn add_session(&self, session: Session) {
        self.sessions
            .lock()
            .await
            .push(Arc::new(RwLock::new(session)));
    }

    pub async fn get_sessions_info(&self) -> Result<Vec<SessionInfo>> {
        let sessions = self.sessions.lock().await;
        let infos = sessions.iter()
                .map(|s| async { s.read().await.info().clone() });
            
        Ok(futures::future::join_all(infos).await)
    }

    pub async fn get_session(&self, id: usize) -> Option<Arc<RwLock<Session>>> {
        let sessions = self.sessions.lock().await;
        for session in sessions.iter() {
            if session.read().await.id() == id {
                return Some(session.clone());
            }
        }
        None
    }
}
