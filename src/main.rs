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
// src/main.rs
mod lsp;

use lsp::{LspClientBuilder, protocol::requests, types};

#[tokio::main]
async fn main() -> Result<(), lsp::error::LspError> {
    // Build client with builder pattern
    let mut transport = LspClientBuilder::new("nixd")
        .build()
        .await?;

    // Get file paths
    let current_dir = std::env::current_dir()?;
    let root_uri = format!("file://{}", current_dir.to_string_lossy());
    let file_path = current_dir.join("example.nix");
    let file_uri = format!("file://{}", file_path.to_string_lossy());

    // Initialize using protocol types
    let init_params = requests::InitializeParams {
        process_id: std::process::id(),
        root_uri: root_uri.clone(),
        capabilities: serde_json::json!({
            "textDocument": {
                "documentSymbol": {
                    "hierarchicalDocumentSymbolSupport": true
                }
            }
        }),
    };

    // Send initialization request
    let init_message = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": init_params,
    });
    
    transport.send_raw(init_message.to_string()).await?;
    
    // Read initialization response
    let init_response = transport.receive_raw().await?;
    println!("Server capabilities:\n{}", serde_json::to_string_pretty(&init_response)?);

    // Send initialized notification
    let initialized_msg = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    transport.send_raw(initialized_msg.to_string()).await?;

    // Open document
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
    transport.send_raw(did_open_msg.to_string()).await?;

    // Wait for server to process didOpen (no formal response required)
    let _ = transport.receive_raw().await?;

    // Request document symbols
    let symbol_params = requests::DocumentSymbolParams {
        text_document: types::TextDocumentIdentifier { uri: file_uri.clone() }
    };
    let symbol_request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/documentSymbol",
        "params": symbol_params
    });
    transport.send_raw(symbol_request.to_string()).await?;

    // Get symbol response
    let symbol_response = transport.receive_raw().await?;
    println!("Document symbols:\n{}", serde_json::to_string_pretty(&symbol_response)?);

    // Send shutdown request
    let shutdown_msg = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "shutdown",
        "params": null
    });
    transport.send_raw(shutdown_msg.to_string()).await?;
     
    // Get shutdown response
    let shutdown_response = transport.receive_raw().await?;
    println!("Shutdown response: {}", shutdown_response);

    // Send exit notification
    let exit_msg = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "exit",
        "params": null
    });
    transport.send_raw(exit_msg.to_string()).await?;

    Ok(())
}