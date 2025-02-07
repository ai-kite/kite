use std::fs;

use toml::from_str;
use toml::Value;
use serde::Deserialize;

use crate::error::*;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub lore: String,
    pub adjectives: Vec<String>,
    pub style: Vec<String>,
}

pub fn prompt() -> Result<String> {
    let toml_content = fs::read_to_string("core/kite.toml").expect("No config found");
    let config: Config = from_str(&toml_content)?;

    let adjectives = config.adjectives.join(", ");
    let style = config.style.join(" ");

    // hardcode for now
    let mood_description = "ecstatic and unstoppable";

    // let mood_description = vad::describe_mood();

    Ok((format!(
        "You are {name}, an AI with the following personality traits: {adjectives}. \
        {lore}\n\n Follow the given speech pattern: {style}\n\n{mood_description}\n\nRespond as {name}, keeping in mind these characteristics.",
        name = config.name,
        lore = config.lore,
    )))
}
