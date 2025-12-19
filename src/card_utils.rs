
use rand::Rng;

/*--------------------------------*\
: necessary constants
\*--------------------------------*/

const COLOR_OPTIONS: [char; 4] = ['r','y','b','g'];
const NUMBER_OPTIONS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
const INITIAL_DECK_SIZE: u8 = 3;

/*------------------------------*\
: card struct & functions setup
\*------------------------------*/

/* add attributes of card */
struct card {
    color: char,
    number: char,
}

/* add functions relating to an individual card */
impl card {
    /* check if self matches other card */
    fn does_card_match(card: &String) -> bool {
        if self.color == card.color || self.color == card.color {
            return true;
        } else {return false;}
    }
    
    /* display a single given card with ascii */
    pub fn display_single_card(card: &str) {
        /* setup variables for the color and number based on the card */
        let color = card.chars().nth(0).unwrap();
        let num = card.chars().nth(1).unwrap().to_digit(10).unwrap() as i32;
    
        /* print out a card */
        println!("/-------\\ 
|{num}      | 
|       | 
|   {color}   | 
|       | 
|      {num}| 
\\-------/ ");
    }
}

/*--------------------------------*\
: essential card related utilities
\*--------------------------------*/

pub fn pull_card() -> String {
    /* initialize rng and list the card gets made in */
    let mut rng = rand::thread_rng();
    let mut card_generating_list: Vec<char> = Vec::new();

    /* generating the card */
    card_generating_list.push(COLOR_OPTIONS[rng.gen_range(0..4)]);
    card_generating_list.push(NUMBER_OPTIONS[rng.gen_range(0..9)]);

    let card: String = card_generating_list.iter().collect();

    return card;
}

pub fn generate_deck() -> Vec<String> {
    /* initializing and creating deck */
    let mut deck: Vec<String> = Vec::new();
    while deck.len() < INITIAL_DECK_SIZE.into() {

        /* adding a single card */
        deck.push(pull_card());
    }
    return deck;
}

pub fn remove_card_from_deck(deck: &mut Vec<String>, card: &String) {
    let original_length = deck.len();
    /* retain all cards that dont match the target and remove the target card */
    deck.retain(|card_being_checked| card_being_checked != card);

    if original_length == deck.len() {
        println!("DECK SIZE DIDNT CHANGE");
    }
}
