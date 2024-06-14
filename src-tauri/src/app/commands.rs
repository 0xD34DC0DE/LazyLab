use serde::{Deserialize, Serialize};

use super::AppState;
use crate::ssh::{Host, Interface, Session, SessionInfo};

pub struct CmdError(anyhow::Error);

impl Serialize for CmdError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

pub type CmdResult<T> = Result<T, CmdError>;

impl From<anyhow::Error> for CmdError {
    fn from(e: anyhow::Error) -> Self {
        Self(e)
    }
}

#[derive(Deserialize)]
pub struct SessionStartRequest {
    host: Host,
    user: String,
    password: String,
}

#[tauri::command]
pub async fn start_session(
    req: SessionStartRequest,
    app_state: tauri::State<'_, AppState>,
) -> CmdResult<usize> {
    let session = Session::connect(req.host.into(), req.user, req.password).await?;
    let id = session.id();
    app_state.add_session(session).await;
    Ok(id)
}

#[tauri::command]
pub async fn get_sessions(app_state: tauri::State<'_, AppState>) -> CmdResult<Vec<SessionInfo>> {
    Ok(app_state.get_sessions_info().await?)
}

#[tauri::command]
pub async fn get_interfaces(
    session_id: usize,
    app_state: tauri::State<'_, AppState>,
) -> CmdResult<Vec<Interface>> {
    todo!()
}
