use std::sync::Arc;

use anyhow::{Context, Result};
use rand::Rng;
use russh::{client, ChannelMsg};
use serde::{Deserialize, Serialize};

use super::{client::Client, interface::Interface};

#[derive(Deserialize)]
pub struct Host {
    address: String,
    port: u16,
}

impl Host {
    fn new(address: String, port: u16) -> Self {
        Self { address, port }
    }
}

impl Into<(String, u16)> for Host {
    fn into(self) -> (String, u16) {
        (self.address, self.port)
    }
}

#[derive(Serialize, Clone)]
pub struct SessionInfo {
    id: usize,
    user: String,
    addrs: (String, u16),
}

impl SessionInfo {
    pub fn new(user: String, addrs: (String, u16)) -> Self {
        Self { id: rand::thread_rng().gen(), user, addrs }
    }
}

pub struct Session {
    info: SessionInfo,
    session: client::Handle<Client>,
}

impl Session {
    pub fn id(&self) -> usize {
        self.info.id
    }

    pub fn info(&self) -> &SessionInfo {
        &self.info
    }
}

impl Session {
    pub async fn connect(addrs: (String, u16), user: String, password: String) -> Result<Session> {
        let config = Arc::new(client::Config::default());
        let sh = Client {};
        let mut session = client::connect(config, &addrs, sh).await?;
        let auth_res = session
            .authenticate_password(user.as_str(), password)
            .await?;
        if !auth_res {
            return Err(anyhow::anyhow!("authentication failed"));
        };

        Ok(Self {
            session,
            info: SessionInfo::new(
                user,
                addrs
            ),
        })
    }
}
