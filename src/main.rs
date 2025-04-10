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

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct InitializeParams {
    process_id: u32,
    root_uri: String,
    capabilities: serde_json::Value,
}

fn to_lsp_message(json: serde_json::Value) -> String {
    let body = json.to_string();
    format!("Content-Length: {}\r\n\r\n{}", body.len(), body)
}

async fn send_lsp(stdin: &mut tokio::process::ChildStdin, msg: serde_json::Value) {
    let lsp_msg = to_lsp_message(msg);
    stdin.write_all(lsp_msg.as_bytes()).await.unwrap();
}

async fn read_lsp_message(reader: &mut BufReader<tokio::process::ChildStdout>) -> serde_json::Value {
    let mut content_length = 0;
    let mut line = String::new();

    loop {
        line.clear();
        reader.read_line(&mut line).await.unwrap();
        if line == "\r\n" {
            break;
        }
        if line.to_ascii_lowercase().starts_with("content-length:") {
            let parts: Vec<&str> = line.split(':').collect();
            content_length = parts[1].trim().parse::<usize>().unwrap();
        }
    }

    let mut body = vec![0u8; content_length];
    reader.read_exact(&mut body).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start the nixd language server
    let mut child = Command::new("nixd")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    // Get absolute file URI
    let file_path: PathBuf = std::env::current_dir()?.join("example.nix");
    let file_uri = format!("file://{}", file_path.to_string_lossy());

    // Initialize
    let params = InitializeParams {
        process_id: std::process::id(),
        root_uri: format!("file://{}", std::env::current_dir()?.to_string_lossy()),
        capabilities: serde_json::json!({}),
    };
    let init_msg = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": params,
    });
    send_lsp(&mut stdin, init_msg).await;
    let response = read_lsp_message(&mut reader).await;
    println!("Init response: {}", response);

    // didOpen with real file contents
    let content = std::fs::read_to_string(&file_path)?;
    let did_open_msg = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": file_uri,
                "languageId": "nix",
                "version": 1,
                "text": content
            }
        }
    });
    send_lsp(&mut stdin, did_open_msg).await;
    let _ = read_lsp_message(&mut reader).await; // ignore didOpen response if any

    // textDocument/documentSymbol
    let document_symbol_msg = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/documentSymbol",
        "params": {
            "textDocument": {
                "uri": file_uri
            }
        }
    });
    send_lsp(&mut stdin, document_symbol_msg).await;
    let symbol_response = read_lsp_message(&mut reader).await;
    let pretty = serde_json::to_string_pretty(&symbol_response)?;
    println!("Document symbols:\n{}", pretty);


    // Clean shutdown
    let shutdown_msg = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "shutdown",
        "params": null
    });
    send_lsp(&mut stdin, shutdown_msg).await;
    let shutdown_response = read_lsp_message(&mut reader).await;
    println!("Shutdown response: {}", shutdown_response);

    let exit_msg = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "exit",
        "params": null
    });
    send_lsp(&mut stdin, exit_msg).await;

    Ok(())
}