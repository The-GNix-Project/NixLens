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

pub trait LspRequest {
    type Response: serde::de::DeserializeOwned;
    const METHOD: &'static str;
    fn build_params(self, uri: String, version: u64) -> Value;
}

pub trait LspNotification {
    const METHOD: &'static str;
    fn build_params(self, uri: String, version: u64) -> Value;
}

// Text Document Synchronization
#[derive(Debug)]
pub struct DidOpenTextDocument {
    pub text: String,
    pub language_id: String,
}

impl LspNotification for DidOpenTextDocument {
    const METHOD: &'static str = "textDocument/didOpen";

    fn build_params(self, uri: String, version: u64) -> Value {
        serde_json::json!({
            "textDocument": {
                "uri": uri,
                "languageId": self.language_id,
                "version": version,
                "text": self.text
            }
        })
    }
}

// Hover Request
#[derive(Debug)]
pub struct HoverRequest {
    pub position: (u64, u64),
}

impl LspRequest for HoverRequest {
    type Response = Option<super::Hover>;
    const METHOD: &'static str = "textDocument/hover";

    fn build_params(self, uri: String, _version: u64) -> Value {
        serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": self.position.0, "character": self.position.1 }
        })
    }
}

// Hover Response Structure
#[derive(Debug, serde::Deserialize)]
pub struct Hover {
    pub contents: Vec<MarkedString>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum MarkedString {
    String(String),
    LanguageString { language: String, value: String },
}