#![allow(unused)]

pub mod error;

use error::*;

use common::env;

use reqwest::Client;
use serde_json::json;

// TODO: Too bloated, make your own.
use google_generative_ai_rs::v1::{
    api::Client as OtherClient,
    gemini::{request::Request, Content, Part, Role},
};

#[tokio::main]
pub async fn openai_gen(content: String) -> Result<()> {
    let api_key: String = match env::var("OPENAI_API_KEY") {
        Ok(v) => v,
        Err(e) => return Err(Error::Env(e)),
    };

    let payload = json!({
        "model": "gpt-3",
        "max_tokens": 150,
        "messages": [{
            "role": "system",
            "content":  "{content}"
        }]
    });

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await?;


    let response_text = response.text().await?;
    let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

    // dbg!(&response_json);

    if response_json.get("error").is_some() {
        let error_message = response_json["error"]["message"]
            .as_str()
            .unwrap_or("Unknown error");
        eprintln!("{}", &error_message);
    }

    let response_content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No response from OpenAI");

    dbg!(&response_content);

    Ok(())
}

#[tokio::main]
pub async fn gemini_gen(content: String) -> Result<()> {
    let api_key: String = match env::var("GOOGLE_GENERATIVE_AI_API_KEY") {
        Ok(v) => v,
        Err(e) => return Err(Error::Env(e)),
    };

    let client = OtherClient::new(api_key);
    let txt_request = Request {
        contents: vec![Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(content),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        }],
        tools: vec![],
        safety_settings: vec![],
        generation_config: None,
    };

    let response = client.post(30, &txt_request).await?;

    dbg!(&response);

    Ok(())
}
