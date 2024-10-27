use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct Candidates {
    pub commands: Vec<String>,
    pub aliases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionQuery {
    pub token: String,
    pub word: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResult {
    pub label: String,
    pub help: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Help {
    pub command: String,
    pub help: String,
}