
/* bring in card_utils */
use crate::card_utils;
use crate::card_utils::Card;

/*--------------------------------*\
: necessary constants
\*--------------------------------*/

const CARDS_PER_ROW: u8 = 7;

/*--------------------------------*\
: essential ascii display functions
\*--------------------------------*/

/* display 1 text line of a row of cards */
pub fn display_line_of_cards(deck: &Vec<Card>, line_type: &str, info_placement: &str, row: u8) {
    /* setup variables necessary to display and have info about the card */
    let mut cards_left = deck.len() as u8;
    cards_left -= (row - 1) * CARDS_PER_ROW;
    
    /* initialize based on what row its on */
    let mut current_card = (row - 1) * CARDS_PER_ROW;

    /* loop while there are cards remaining until the (amount of cards in a row) have been printed */
    while cards_left > 0 {        

        let card = &deck[current_card as usize];

        /* print out that type of line with the info it needs */ 
        if line_type == "num" && info_placement == "left" {
            print!("|{}      |  ", card.number);
        } else if line_type == "num" && info_placement == "right" {
            print!("|      {}|  ", card.number);
        } else if line_type == "color" {
            print!("|   {}   |  ", card.color);
        } else if line_type == "none" {
            print!("|       |  ");
        } else if line_type == "top" {
            print!("/-------\\  ");
        } else if line_type == "bottom" {
            print!("\\-------/  ");
        }

        cards_left -= 1;
        current_card += 1;

        /* break when its done with the row */
        if current_card > row * CARDS_PER_ROW - 1 {
            break;
        }
    }
    print!("\n");
}

/* display a full deck of cards */
pub fn display_player_deck(deck: &Vec<String>) {

    let mut card_row = 1;

    /* loop and display ascii for every card in the deck */
    while card_row <= deck.len().div_ceil(CARDS_PER_ROW.into()) { 
        
        display_line_of_cards(deck, "top", "", card_row as u8);
        display_line_of_cards(deck, "num", "left", card_row as u8);
        display_line_of_cards(deck, "none", "", card_row as u8);
        display_line_of_cards(deck, "color", "", card_row as u8);
        display_line_of_cards(deck, "none", "", card_row as u8);
        display_line_of_cards(deck, "num", "right", card_row as u8);
        display_line_of_cards(deck, "bottom", "", card_row as u8);

        println!(" ");

        card_row += 1;
    }
}
