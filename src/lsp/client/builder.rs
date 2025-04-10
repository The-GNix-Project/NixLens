// SPDX-License-Identifier: GPL-3.0-or-later
//
// This file is part of GNix.
// GNix - The Graphical Nix Project
// -----------------------------------------------------------------------------------------|
// GNix is free software: you can redistribute it and/or modify                             |
// it under the terms of the GNU General Public License as published by                     |
// the Free Software Foundation, either version 3 of the License, or any later version.     |
//                                                                                          |
// GNix is distributed in the hope that it will be useful,                                  |
// but WITHOUT ANY WARRANTY; without even the implied warranty of                           |
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                            |
// GNU General Public License for more details.                                             |
//                                                                                          |
// You should have received a copy of the GNU General Public License                        |
// along with GNix.  If not, see <https://www.gnu.org/licenses/>.                           |
// -----------------------------------------------------------------------------------------|
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