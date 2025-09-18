use std::env;

use serde::{Deserialize, Serialize};
use anyhow::Context;

use crate::prompt::PROMPT;

const RESPONSES_ENDPOINT: &str = "https://api.openai.com/v1/responses";


pub async fn get_response(query: &str) -> anyhow::Result<String> {
    let req = ResponsesRequest::new_from_query(query);
    let t = get_token()?;
    let c = reqwest::Client::new();
    let resp = c.post(RESPONSES_ENDPOINT)
        .bearer_auth(t)
        .json(&req)
        .send()
        .await?;

    let status = resp.status();
    let body = resp
        .text()
        .await
        .context("failed to read response body")?;

    if !status.is_success() {
        // Include status + body to make debugging easy
        anyhow::bail!("Responses API error ({}): {}", status, body);
    }

    let oai_resp: ResponsesResponse = serde_json::from_str(&body)
        .with_context(|| format!("failed to deserialize success body: {}", body))?;

    let text = oai_resp
        .find_text_response()
        .ok_or_else(|| anyhow::anyhow!("Could not find a response from OAI"))?;

    Ok(text)
}

fn get_token() -> anyhow::Result<String> {
    env::var("OPENAI_API_KEY")
        .with_context(|| "OPENAI_API_KEY not set in environment")
}

#[derive(Serialize, Deserialize)]
struct ResponsesRequest {
    model: String,
    input: String,
    instructions: String,
}

impl ResponsesRequest {
    fn new_from_query(query: &str) -> Self {
        ResponsesRequest {
            model: "gpt-4o".to_owned(),
            input: query.to_owned(),
            instructions: PROMPT.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ResponsesResponse {
    output: Vec<ResponsesOutput>
}

impl ResponsesResponse {
    fn find_text_response(&self) -> Option<String> {
        self.output
            .iter()
            .find(|o| o.output_type == "message")
            .and_then(|o| {
                o.content
                    .iter()
                    .find(|c| c.content_type == "output_text")
                    .map(|c| c.text.clone())
            })
    }
}

#[derive(Serialize, Deserialize)]
struct ResponsesOutput {
    #[serde(rename = "type")]
    output_type: String,
    content: Vec<ResponsesContent>,
}

#[derive(Serialize, Deserialize)]
struct ResponsesContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

