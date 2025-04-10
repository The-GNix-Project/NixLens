use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InitializeResult {
    pub capabilities: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct DocumentSymbolResponse {
    // Define structure based on LSP spec
}

// Add other response types as needed