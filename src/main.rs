extern crate ureq;
extern crate serde_json;
extern crate serde;
extern crate dotenv_codegen;
extern crate rand;

use dotenv_codegen::dotenv;
use std::io::{stdin, stdout, Write};
use serde::{Deserialize};
use serde_json::Result;
use std::char;
use rand::*;

static ACCESS_KEY: &str = dotenv!("OPEN_EMOJI_KEY");

fn clean_input(input: &mut String) {
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
}

fn get_search() -> String {
    let mut emoji_search = String::new();

    let x: u32 = thread_rng().gen_range(0x1F600, 0x1F64F);
    let emoji = char::from_u32(x).unwrap_or('💔');

    print!("{} → ", emoji);

    stdout().flush().expect("ugh");
    stdin().read_line(&mut emoji_search).expect("ugh");
    clean_input(&mut emoji_search);

    emoji_search
}


fn build_request_url(search: &str) -> String {
    return format!("https://emoji-api.com/emojis?search={}&access_key={}", search, ACCESS_KEY)
} 

#[derive(Deserialize)]
struct Emoji {
    character: String,
}

fn handle_response(response: &str) -> Result<()> {
    let emojis: Vec<Emoji> = serde_json::from_str(response)?;
    let amount = emojis.len();

    let s = if amount > 1 { "s" } else { "" };
    println!("Found {} result{} \n", amount, s);

    if amount == 0 {
        println!("ADASDASD");
        return Ok(())
    }

    for emoji in emojis {
        println!("{}", emoji.character);
    }

    Ok(())
}

fn main() {
    let emoji_search = get_search();

    let formatted_request_url: String = build_request_url(emoji_search.as_str());

    let mut request = ureq::get(formatted_request_url.as_str());
    let response = request.call();

    if response.ok() {
        let unformatted_matches = response.into_string().unwrap();

        match handle_response(unformatted_matches.as_str()) {
            Ok(_) => (),
            Err(_) => println!("0 results found"),
        }
    } else {
        println!("Something went wrong");
    }
}
