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
use tokio::process::{Command, Child};
use super::{LspError, Cursor, LspTransport};

pub struct Connection {
    transport: LspTransport,
    _process: Child,
}

impl Connection {
    pub async fn new() -> Result<Self, LspError> {
        let mut process = Command::new("nixd")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| LspError::Server(format!("Failed to start nixd: {}", e)))?;

        let stdin = process.stdin.take().ok_or_else(|| 
            LspError::Protocol("Failed to get stdin handle".into())
        )?;

        let stdout = process.stdout.take().ok_or_else(|| 
            LspError::Protocol("Failed to get stdout handle".into())
        )?;

        Ok(Self {
            transport: LspTransport::new(stdin, stdout),
            _process: process,
        })
    }

    pub async fn initialize(&mut self) -> Result<(), LspError> {
        let init_params = serde_json::json!({
            "processId": std::process::id(),
            "clientInfo": {
                "name": "nixd-client",
                "version": "0.1.0"
            },
            "rootUri": "file:///home/archie/GNix/NixLens/",
            "capabilities": {
                "workspace": {
                    "workspaceFolders": true,
                    "configuration": true
                },
                "textDocument": {
                    "hover": {
                        "contentFormat": ["markdown", "plaintext"]
                    },
                    "synchronization": {
                        "didSave": true,
                        "dynamicRegistration": false
                    },
                    "completion": {
                        "completionItem": {
                            "snippetSupport": false
                        }
                    }
                }
            },
            "initializationOptions": {
                "enableNixpkgsLib": true,
                "formattingCommand": "nixpkgs-fmt"
            },
            "trace": "verbose"
        });

        let response = self.transport.send_request("initialize", init_params)
            .await?;

        tracing::debug!("Server capabilities: {}", response);
        self.transport.send_notification("initialized", serde_json::json!({}))
            .await
    }

    pub fn cursor(&mut self, uri: String) -> Cursor {
        Cursor::new(&mut self.transport, uri)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        let _ = self.transport.send_notification("shutdown", serde_json::Value::Null);
        let _ = self.transport.send_notification("exit", serde_json::Value::Null);
    }
}