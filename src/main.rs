use serde::{Deserialize, Serialize};
use serde_json;

use std::fs::File;
use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
struct WhiteCard {
    #[serde(default)]
    text: String,
    pack: u8,
}

#[derive(Deserialize, Serialize, Debug)]
struct BlackCard {
    #[serde(default)]
    text: String,
    pack: u8,
    pick: u8,
}

#[derive(Deserialize, Serialize, Debug)]
struct CardPack {
    #[serde(default)]
    name: String,
    official: bool,
    white: Vec<WhiteCard>,
    black: Vec<BlackCard>
}

fn main() {
    read_json().unwrap();
}

pub fn read_json() -> Result<(), Box<dyn Error>> {
    let players = vec!["Player 1", "Player 2", "Player 3", "Player 4"]; 
    let mut black_cards_text: Vec<String> = Vec::new();
    let mut white_cards_text: Vec<String> = Vec::new();

    let file = File::open("src/assets/json/full.json")?;

    let pack: Vec<CardPack> = serde_json::from_reader(file)?;
    
    for val in pack {
        for card in val.black {
            black_cards_text.push(card.text);
        }
        for card in val.white {
            white_cards_text.push(card.text);
        }
    }

    println!("{}", black_cards_text.len());
    println!("{}", white_cards_text.len());
    
    Ok(())
}
