// `core` should be where we assemble kite and where the main function should
// live.

// should consist glue code that brings crates together in order to execute
// itself.

#![allow(unused)]

mod error;
mod mood;
mod system;

use crate::error::*;
use crate::system::prompt;

use std::io::{self, Write};

use indexer;
use llm;

fn main() -> Result<()> {
    loop {
        print!("\nYou: ");

        let _ = io::stdout().flush().unwrap();

        let mut user_input = String::new();
        if io::stdin().read_line(&mut user_input).is_err() {
            eprintln!("Error reading input.");
            continue;
        }

        let user_input = user_input.trim();
        if user_input.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        println!("\nwaiting for response...\n");

        print!("kite: ");
        llm::gemini_gen(prompt()?, user_input.to_string())?
    }

    Ok(())
}
