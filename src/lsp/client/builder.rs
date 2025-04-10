use super::{LspTransport, super::error::LspError};
use tokio::process::Command;

pub struct LspClientBuilder {
    server_path: String,
    initialization_options: Option<serde_json::Value>,
}

impl LspClientBuilder {
    pub fn new(server_path: &str) -> Self {
        Self {
            server_path: server_path.to_string(),
            initialization_options: None,
        }
    }

    pub fn with_initialization_options(mut self, options: serde_json::Value) -> Self {
        self.initialization_options = Some(options);
        self
    }

    pub async fn build(self) -> Result<LspTransport, LspError> {
        let mut child = Command::new(&self.server_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().ok_or(LspError::Protocol("Failed to acquire stdin".into()))?;
        let stdout = child.stdout.take().ok_or(LspError::Protocol("Failed to acquire stdout".into()))?;

        Ok(LspTransport::new(stdin, stdout))
    }
}