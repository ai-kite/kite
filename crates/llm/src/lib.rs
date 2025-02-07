#![allow(unused)]

pub mod error;

use error::*;

use common::env;

use reqwest::Client;
use serde_json::json;

// TODO: Too bloated, make your own.
use google_generative_ai_rs::v1::{
    api::{Client as OtherClient, PostResult},
    gemini::{
        request::{GenerationConfig, Request},
        Content,
        Part,
        Role,
    },
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
pub async fn gemini_gen(system: String, text: String) -> Result<()> {
    let api_key: String = match env::var("GOOGLE_GENERATIVE_AI_API_KEY") {
        Ok(v) => v,
        Err(e) => return Err(Error::Env(e)),
    };

    let client = OtherClient::new(api_key);
    let txt_request = Request {
        contents: vec![
            Content {
                role: Role::Model,
                parts: vec![Part {
                    text: Some(system),
                    inline_data: None,
                    file_data: None,
                    video_metadata: None,
                }],
            },
            Content {
                role: Role::User,
                parts: vec![Part {
                    text: Some(text),
                    inline_data: None,
                    file_data: None,
                    video_metadata: None,
                }],
            },
        ],
        tools: vec![],
        safety_settings: vec![],
        generation_config: None,
    };

    let post_result = client.post(30, &txt_request).await?;

    // TODO: temp structure
    if let PostResult::Rest(response) = post_result {
        if let Some(candidate) = response.candidates.get(0) {
            if let Some(part) = candidate.content.parts.get(0) {
                if let Some(text) = &part.text {
                    println!("{}", &text);
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn arli_gen(system: String, user_input: String) -> Result<()> {
    let api_key: String = match env::var("ARLIAI_API_KEY") {
        Ok(v) => v,
        Err(e) => return Err(Error::Env(e)),
    };

    let url = "https://api.arliai.com/v1/chat/completions";
    let client = Client::new();

    let mut messages = vec![json!({"role": "system", "content": system})];

    messages.push(json!({"role": "user", "content": user_input}));

    let payload = json!({
        "model": "Mistral-Nemo-12B-Instruct-2407",
        "messages": messages,
        "repetition_penalty": 1.1,
        "temperature": 0.7,
        "top_p": 0.9,
        "top_k": 40,
        "max_tokens": 1024,
        "stream": false
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;
    if let Some(reply) = response_json["choices"][0]["message"]["content"].as_str() {
        println!("kite: {}", reply);
        messages.push(json!({"role": "kite", "content": reply}));
    } else {
        println!("Error: No response from AI.");
    }

    Ok(())
}
