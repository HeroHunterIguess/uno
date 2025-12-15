
use rand::Rng;

/*--------------------------------*\
: necessary constants
\*--------------------------------*/

const COLOR_OPTIONS: [char; 4] = ['r','y','b','g'];
const NUMBER_OPTIONS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
const INITIAL_DECK_SIZE: u8 = 1;

/*--------------------------------*\
: essential card related utilities
\*--------------------------------*/

pub fn get_card_info(card: &str) -> (char, char) {
    /* get color and number of current card */
    let color = card.chars().nth(0).unwrap();
    let num = card.chars().nth(1).unwrap();
    return (color, num);
}

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

pub fn does_card_match(card_1: &String, card_2: &String) -> bool {
    /* check if colors match */
    if card_1.chars().nth(0) == card_2.chars().nth(0) {
        return true;
    
    /* check if numbers match */
    } else if card_1.chars().nth(1) == card_2.chars().nth(1) {
        return true;
    }
    return false;
}

pub fn remove_card_from_deck(deck: &mut Vec<String>, card: &String) {
    let original_length = deck.len();
    /* retain all cards that dont match the target and remove the target card */
    deck.retain(|card_being_checked| card_being_checked != card);

    if original_length == deck.len() {
        println!("DECK SIZE DIDNT CHANGE");
    }
}
