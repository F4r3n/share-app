use anyhow::{anyhow, Ok};

use crate::completion::types::{Candidates, CompletionQuery, CompletionResult};

use super::types::Help;

pub async fn complete(
    endpoint: &str,
    token: &str,
    word: &str,
) -> Result<Vec<CompletionResult>, anyhow::Error> {
    let query = CompletionQuery {
        token: token.to_string().clone(),
        word: word.into(),
    };
    let candidates = send_completion_query(endpoint, query).await?;

    let matches: Vec<CompletionResult> = candidates
        .commands
        .iter()
        .chain(candidates.aliases.iter())
        .map(|label| CompletionResult {
            label: label.to_string(),
            help: None
        })
        .collect();

    Ok(matches)
}

async fn send_completion_query(
    endpoint: &str,
    query: CompletionQuery,
) -> Result<Candidates, anyhow::Error> {
    let client = reqwest::Client::new();
    let request = client
        .post(format!("{}/completions", endpoint))
        .json(&query);
    let response = request.send().await?.error_for_status()?;
    response.json().await.map_err(|err| anyhow!(err))
}

pub async fn help(endpoint: &str, token: &str, word: &str) -> Result<String, anyhow::Error> {
    let query = CompletionQuery {
        token: token.to_string().clone(),
        word: word.into(),
    };

    let help = send_help_query(endpoint, query).await?;
    Ok(help.help.to_string())
}

async fn send_help_query(endpoint: &str, query: CompletionQuery) -> Result<Help, anyhow::Error> {
    let client = reqwest::Client::new();
    let request = client.post(format!("{}/help", endpoint)).json(&query);
    let response = request.send().await?.error_for_status()?;
    response.json().await.map_err(|err| anyhow!(err))
}
