// `core` should be where we assemble kite and where the main function should
// live.

// should consist glue code that brings crates together in order to execute
// itself.

#![allow(unused)]

mod error;
mod vad;

use error::*;
use std::fs;

use indexer;
use llm;

fn main() -> Result<()> {
    // if let Err(e) = indexer::run() {
    //     eprintln!("{}", e);
    //     return Err(Error::Indexer(e))
    // };

    let content = fs::read_to_string("core/kite.toml").expect("lmao");
    if let Err(e) = llm::gemini_gen(content) {
        eprintln!("{}", e);
        return Err(Error::LLM(e));
    }

    // vad::calc_mood();

    Ok(())
}
