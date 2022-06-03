use colored::*;
use serde::{Deserialize, Serialize};
use serde_json;

use rand::prelude::*;
use std::error::Error;
use std::fs::File;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WhiteCard {
    #[serde(default)]
    text: String,
    pack: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlackCard {
    #[serde(default)]
    text: String,
    pack: u8,
    pick: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Deck {
    #[serde(default)]
    name: String,
    official: bool,
    white: Vec<WhiteCard>,
    black: Vec<BlackCard>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Player {
    #[serde(default)]
    name: String,
    cards: Vec<WhiteCard>,
}

// main function
fn main() {
    start_game();
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

// get messages
pub fn get_messages() -> [String; 5] {
    let mut messages: [String; 5] = [
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    ];

    messages[0] = "Welcome to the game!".to_string().to_uppercase();
    messages[1] = "Please enter the players name separated by coma:"
        .to_string()
        .to_uppercase();
    messages[2] = "Please enter the number of the deck you want to play:"
        .to_string()
        .to_uppercase();
    messages[3] = "The players are:".to_string().to_uppercase();
    messages[4] = "The turn is for:".to_string().to_uppercase();

    messages
}

// game start
pub fn start_game() {
    let get_decks_result: Result<Vec<Deck>, Box<dyn Error>> = get_decks();
    let decks: Vec<Deck> = get_decks_result.unwrap();
    let mut white_cards: Vec<WhiteCard>;
    let black_cards: Vec<BlackCard>;

    let messages: [String; 5] = get_messages();
    let mut players: Vec<Player> = Vec::new();
    // insert players names
    println!("{}", messages[0].bold().green());
    println!("{}", messages[1].bold().green());
    let players_input = get_input();
    let str_players: Vec<&str> = players_input.split(',').collect();
    // push names to vec with struct
    for name in str_players {
        players.push(Player {
            name: name.to_string(),
            cards: Vec::new(),
        });
    }
    // show a list of available decks
    for (pos, e) in decks.iter().enumerate() {
        println!("{}: {}", pos.to_string().green(), e.name.blue());
    }
    println!("{}", messages[2].bold().green());
    // clears the terminal
    print!("\x1B[2J\x1B[1;1H");

    let deck_index = get_input();
    let selected_deck: &Deck = decks.get(deck_index.parse::<usize>().unwrap()).unwrap();
    // select each cards from the deck
    white_cards = selected_deck.white.clone();
    black_cards = selected_deck.black.clone();

    let selected_black_card = black_cards.choose(&mut thread_rng()).unwrap();

    // suffle 4 white cards for each player
    for player in players.iter_mut() {
        let mut player_cards: Vec<WhiteCard> = Vec::new();
        for _ in 0..4 {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..white_cards.len());
            player_cards.push(white_cards.remove(index));
        }
        player.cards = player_cards;

        println!(
            "{}{}",
            "THE TURN IS FOR:".bold().green(),
            player.name.cyan().bold()
        );
        println!(
            "YOU'RE PLAYING WITH THIS BLACK CARD: {}",
            selected_black_card.text.yellow().bold()
        );
        println!(
            "THE NUMBER OF CARDS TO BE USED IS: {}",
            selected_black_card.pick.to_string().yellow().bold()
        );
        println!(
            "{}",
            "PLEASE SELECT A CARD TO PLAY ENTERING THE CORRESPONDING NUMBER:"
                .bold()
                .green()
        );

        for (pos, card) in player.cards.iter().enumerate() {
            println!("{}: {}", pos, card.text.magenta().italic());
        }

        let picked_card_index = get_input();
        let picked_card = player
            .cards
            .get(picked_card_index.parse::<usize>().unwrap())
            .unwrap();

        println!("{}{}", "YOU PICKED:".bold().green(), picked_card.text);
        println!("");
    }
}

pub fn get_input() -> String {
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            return input.trim().split_whitespace().collect();
        }
        Err(error) => {
            println!("Error: {}", error);
            String::new()
        }
    }
}
