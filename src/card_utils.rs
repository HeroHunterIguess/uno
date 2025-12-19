
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
struct Card {
    color: char,
    number: char,
}

/* add functions relating to an individual card */
impl Card {
    /* check if self matches other card */
    fn does_card_match(&self, card: &Card) -> bool {
        if self.color == card.color || self.number == card.number {
            return true;
        } else {return false;}
    }
    
    /* display a single given card with ascii */
    fn display_card(&self) {
        /* print out a card */
        println!("/-------\\ 
|{}      | 
|       | 
|   {}   | 
|       | 
|      {}| 
\\-------/ ", self.color, self.number, self.color);
    }
}

/*--------------------------------*\
: essential card related utilities
\*--------------------------------*/

pub fn pull_card() -> Card {
    /* initialize rng and list the card gets made in */
    let mut rng = rand::thread_rng();

    /* generating the card */
    let card = Card {
        color: COLOR_OPTIONS[rng.gen_range(0..COLOR_OPTIONS.len())],
        number: NUMBER_OPTIONS[rng.gen_range(0..NUMBER_OPTIONS.len())],
    };

    return card;
}

pub fn generate_deck() -> Vec<Card> {
    /* initializing and creating deck */
    let mut deck: Vec<Card> = Vec::new();
    while deck.len() < INITIAL_DECK_SIZE.into() {

        /* adding a single card */
        deck.push(pull_card());
    }
    return deck;
}

pub fn remove_card_from_deck(deck: &mut Vec<Card>, card: &Card) {
    let original_length = deck.len();
    /* retain all cards that dont match the target and remove the target card */
    deck.retain(|card_being_checked| card_being_checked != card);

    if original_length == deck.len() {
        println!("WARNING: DECK SIZE DIDNT CHANGE");
    }
}
