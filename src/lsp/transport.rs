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
use serde_json::Value;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{ChildStdin, ChildStdout},
    sync::{mpsc, oneshot, Mutex},
};
use std::{collections::HashMap, sync::Arc};

use super::LspError;

pub struct LspTransport {
    message_tx: mpsc::Sender<String>,
    pending_requests: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    next_id: u64,
}

impl LspTransport {
    pub fn new(stdin: ChildStdin, stdout: ChildStdout) -> Self {
        let (message_tx, message_rx) = mpsc::channel(32);
        let pending_requests = Arc::new(Mutex::new(HashMap::new()));
        let next_id = 1;

        // Spawn writer task
        tokio::spawn(Self::writer_task(stdin, message_rx));
        
        // Spawn reader task
        tokio::spawn(Self::reader_task(stdout, pending_requests.clone()));

        Self {
            message_tx,
            pending_requests,
            next_id,
        }
    }

    async fn writer_task(mut stdin: ChildStdin, mut message_rx: mpsc::Receiver<String>) {
        while let Some(message) = message_rx.recv().await {
            if let Err(e) = stdin.write_all(message.as_bytes()).await {
                eprintln!("Write error: {}", e);
                break;
            }
        }
    }

    async fn reader_task(
        stdout: ChildStdout,
        pending_requests: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    ) {
        let mut reader = BufReader::new(stdout);
        loop {
            match Self::read_message(&mut reader).await {
                Ok(message) => {
                    if let Some(id) = message.get("id").and_then(|id| id.as_u64()) {
                        let mut requests = pending_requests.lock().await;
                        if let Some(sender) = requests.remove(&id) {
                            let _ = sender.send(message);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    break;
                }
            }
        }
    }

    async fn read_message(reader: &mut BufReader<ChildStdout>) -> Result<Value, LspError> {
        let mut content_length = 0;
        let mut headers = String::new();

        // Read headers
        loop {
            headers.clear();
            reader.read_line(&mut headers).await?;
            
            if headers.trim().is_empty() {
                break;
            }
            
            if let Some(len) = headers
                .trim()
                .to_lowercase()
                .strip_prefix("content-length:")
            {
                content_length = len.trim().parse().unwrap_or(0);
            }
        }

        if content_length == 0 {
            return Err(LspError::Protocol("Missing Content-Length".into()));
        }

        // Read body
        let mut body = vec![0u8; content_length];
        reader.read_exact(&mut body).await?;
        serde_json::from_slice(&body).map_err(Into::into)
    }

    pub async fn send_request(&mut self, method: &str, params: Value) -> Result<Value, LspError> {
        let id = self.next_id;
        self.next_id += 1;
        
        let (tx, rx) = oneshot::channel();
        self.pending_requests.lock().await.insert(id, tx);

        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });
        
        self.message_tx.send(msg.to_string())
            .await
            .map_err(|_| LspError::Protocol("Failed to send request".into()))?;
    
        tokio::time::timeout(
            std::time::Duration::from_secs(5), 
            rx
        )
        .await
        .map_err(|_| LspError::Timeout)?
        .map_err(|_| LspError::Protocol("Channel closed".into()))
    }

    pub async fn send_notification(&mut self, method: &str, params: Value) -> Result<(), LspError> {
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        });
        self.message_tx.send(msg.to_string())
            .await
            .map_err(|_| LspError::Protocol("Failed to send notification".into()))
    }
}