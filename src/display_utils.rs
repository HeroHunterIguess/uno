/* any functions without explanations should be self explanatory */

/* bring in card_utils */
use crate::card_utils;

/*--------------------------------*\
: necessary constants
\*--------------------------------*/

const CARDS_PER_ROW: u8 = 7;

/*--------------------------------*\
: essential ascii display functions
\*--------------------------------*/

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

/* display 1 text line of a row of cards */
pub fn display_line_of_cards(deck: &Vec<String>, line_type: &str, info_placement: &str, row: u8) {
    /* setup variables necessary to display and have info about the card */
    let mut cards_left = deck.len() as u8;
    cards_left -= (row - 1) * CARDS_PER_ROW;
    
    /* initialize based on what row its on */
    let mut current_card = (row - 1) * CARDS_PER_ROW;

    /* loop while there are cards remaining until the (amount of cards in a row) have been printed */
    while cards_left > 0 {
        let (color, num) = card_utils::get_card_info(&deck[current_card as usize]);
        
        /* print out that type of line with the info it needs */ 
        if line_type == "num" && info_placement == "left" {
            print!("|{num}      |  ");
        } else if line_type == "num" && info_placement == "right" {
            print!("|      {num}|  ");
        } else if line_type == "color" {
            print!("|   {color}   |  ");
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

pub fn display_general_info(pulled_card: &String, card_on_stack: &String, current_player_deck: &Vec<String>) {
    /* clear screen */
    print!("\x1B[2J");

    /* tell them what they pulled */
    println!("You pulled a {pulled_card}:");
    display_single_card(&pulled_card);

    /* display stack */
    println!("\n\nThis is the card on the stack: ");
    display_single_card(&card_on_stack);

    /* display deck again */
    println!("\nHere is your new deck: ");
    display_player_deck(&current_player_deck);
}

/* display general game info & how to play before game starts */
pub fn display_how_to_play() {
    /* clear screen */
    print!("\x1B[2J");

    /* display info */
    println!("How to play: \n");

    println!("Enter anything to continue...");
}
