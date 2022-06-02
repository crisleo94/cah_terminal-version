use serde::{Deserialize, Serialize};
use serde_json;

use std::fs::File;
use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct WhiteCard {
    #[serde(default)]
    text: String,
    pack: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlackCard {
    #[serde(default)]
    text: String,
    pack: u8,
    pick: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Deck {
    #[serde(default)]
    name: String,
    official: bool,
    white: Vec<WhiteCard>,
    black: Vec<BlackCard>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Player {
    #[serde(default)]
    name: String,
    //cards: Vec<WhiteCard>
}

// main function
fn main() {

}

// get a list of all the decks, each deck contains a different set of cards
pub fn get_decks() -> Result<Vec<Deck>, Box<dyn Error>> {
    let mut decks: Vec<Deck> = Vec::new();
    let file = File::open("src/assets/json/full.json").unwrap();
    let pack: Vec<Deck> = serde_json::from_reader(file).unwrap();

    for val in pack {
        decks.push(val);
    }

    Ok(decks)
}

// game start

pub fn start_game() {
    let get_decks_result: Result<Vec<Deck>, Box<dyn Error>> = get_decks();
    let decks: Vec<Deck> = get_decks_result.unwrap();

    println!("Please enter the players name separated with a coma: ");
    let players_input = get_input();
    let players: Vec<&str> = players_input.split(',').collect();

}

pub fn get_input() -> String {
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            return input.trim().split_whitespace().collect();
        },
        Err(error) => {
            println!("Error: {}", error);
            String::new()
        }
    }
}