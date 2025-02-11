use std::fs;

use serde::Deserialize;
use toml::from_str;
use toml::Value;

use crate::error::*;
use crate::mood;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub lore: String,
    pub adjectives: Vec<String>,
    pub style: Vec<String>,
}

pub fn prompt() -> Result<String> {
    let toml_content = fs::read_to_string("kite.toml").expect("No config found");
    let config: Config = from_str(&toml_content)?;

    let adjectives = config.adjectives.join(", ");
    let style = config.style.join(" ");

    let mood_description = mood::describe();

    dbg!(&mood_description);

    Ok((format!(
        "You are {name}, an AI with the following personality traits: {adjectives}. \
        {lore}\n\n Follow the given speech pattern, your mood can change based on the system prompt: {style}\n\n{mood_description}\n\nRespond as {name}, keeping in mind these characteristics.",
        name = config.name,
        lore = config.lore,
    )))
}
