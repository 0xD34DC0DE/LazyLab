use super::client::Client;
use anyhow::Result;
use russh::{client::Handle, ChannelMsg};

pub trait VirtualCommand<T: 'static, const N: usize> {
    fn implementations(&self) -> [&'static dyn ConcreteCommand<T>; N];

    async fn execute(&self, executor: &impl CommandExecutor) -> Result<T> {
        for implementation in self.implementations() {
            if implementation.detect(executor).await? {
                return implementation.execute(executor).await;
            }
        }
        Err(anyhow::anyhow!("No suitable implementation found"))
    }
}

pub trait ConcreteCommand<T> {
    fn detection_command(&self) -> CommandString;

    fn execution_command(&self) -> CommandString;

    fn parse_detection_output(&self, output: &str) -> Result<bool>;

    fn parse_execution_output(&self, output: &str) -> Result<T>;
}

impl<T> dyn ConcreteCommand<T>
where
    T: Sized,
{
    async fn detect(&self, executor: &impl CommandExecutor) -> Result<bool> {
        let detection_output = executor.execute(self.detection_command().as_str()).await?;
        self.parse_detection_output(&detection_output)
    }

    async fn execute(&self, executor: &impl CommandExecutor) -> Result<T> {
        let output = executor.execute(self.execution_command().as_str()).await?;
        self.parse_execution_output(&output)
    }
}

pub enum CommandString {
    Static(&'static str),
    Dynamic(String),
}

impl CommandString {
    fn as_str(&self) -> &str {
        match self {
            CommandString::Static(s) => s,
            CommandString::Dynamic(s) => s.as_str(),
        }
    }
}

pub trait CommandExecutor {
    async fn execute(&self, command: &str) -> Result<String>;
}

struct SshCommandExecutor {
    handle: Handle<Client>,
}

impl SshCommandExecutor {
    pub fn new(handle: Handle<Client>) -> Self {
        Self { handle }
    }
}

impl CommandExecutor for SshCommandExecutor {
    async fn execute(&self, command: &str) -> Result<String> {
        let mut channel = self.handle.channel_open_session().await?;
        channel.exec(true, command).await?;

        let mut buffer = Vec::new();
        loop {
            let msg = channel
                .wait()
                .await
                .ok_or(anyhow::anyhow!("Channel closed"))?;

            match msg {
                ChannelMsg::Data { data } => {
                    buffer.extend_from_slice(&data);
                }
                ChannelMsg::ExitStatus { exit_status } => {
                    if exit_status != 0 {
                        return Err(anyhow::anyhow!(
                            "Command '{}' failed with exit status {}",
                            command,
                            exit_status
                        ));
                    }
                    break;
                }
                msg => {
                    println!("Unexpected message: {:?}", msg);
                }
            }
        }

        Ok(String::from_utf8(buffer)?)
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use super::*;

    pub struct MockCommandExecutor {
        mappings: HashMap<String, String>,
    }

    impl MockCommandExecutor {
        pub fn new(mappings: HashMap<String, String>) -> Self {
            Self { mappings }
        }
    }

    impl CommandExecutor for MockCommandExecutor {
        async fn execute(&self, command: &str) -> Result<String> {
            self.mappings
                .get(command)
                .cloned()
                .ok_or(anyhow::anyhow!("Command not found"))
        }
    }
}
