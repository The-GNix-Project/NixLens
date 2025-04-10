use serde::Serialize;
use super::super::types::*;

#[derive(Serialize)]
pub struct InitializeParams {
    pub process_id: u32,
    pub root_uri: String,
    pub capabilities: serde_json::Value,
}

#[derive(Serialize)]
pub struct DidOpenTextDocumentParams {
    pub text_document: TextDocumentItem,
}

#[derive(Serialize)]
pub struct DocumentSymbolParams {
    pub text_document: TextDocumentIdentifier,
}

// TODO: add other request params as needed