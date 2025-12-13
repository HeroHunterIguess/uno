/*------------------------------------------------*\
: uno-like game
: created by: Hero
: 
: this is a MAJOR work in progress !!
: its not playable right now !!
: 
: some of this code prob isnt good im bad at rust 
\*------------------------------------------------*/

use rand::Rng;

//setting up necessary constants
const COLOR_OPTIONS: [char; 4] = ['r','y','b','g'];
const NUMBER_OPTIONS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

const CARDS_PER_ROW: u8 = 7;
const INITIAL_DECK_SIZE: u8 = 7;

/*------------------------------*\
: defining essential functions   
\*------------------------------*/

fn get_card_info(card: &str) -> (char, char) {
    //get color and number of current card
    let color = card.chars().nth(0).unwrap();
    let num = card.chars().nth(1).unwrap();
    return (color, num);
}

fn card_ascii(card: &str) {
    //setup variables for the color and number based on the card
    let color = card.chars().nth(0).unwrap();
    let num = card.chars().nth(1).unwrap().to_digit(10).unwrap() as i32;

    //print out a card
    println!("/-------\\ 
|{num}      | 
|       | 
|   {color}   | 
|       | 
|      {num}| 
\\-------/ ");
}

fn display_line_of_cards(deck: &Vec<String>, line_type: &str, info_placement: &str, row: u8) {
    //setup variables necessary to display and have info about the card
    let mut cards_left = deck.len() as u8;
    cards_left -= (row - 1) * CARDS_PER_ROW;
    
    //initialize based on what row its on
    let mut current_card = (row - 1) * CARDS_PER_ROW;

    //loop while there are cards remaining until the (amount of cards in a row) have been printed
    while cards_left > 0 {
        let (color, num) = get_card_info(&deck[current_card as usize]);
        
        //print out that type of line with the info it needs
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

        //break when its done with the row
        if current_card > row * CARDS_PER_ROW - 1 {
            break;
        }
    }
    print!("\n");
}

fn display_player_deck(deck: &Vec<String>) {

    let mut card_row = 1;

    //loop and display ascii for every card in the deck
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

fn pull_card() -> String {
    //initialize rng and list the card gets made in
    let mut rng = rand::thread_rng();
    let mut card_generating_list: Vec<char> = Vec::new();

    //generating the card
    card_generating_list.push(COLOR_OPTIONS[rng.gen_range(0..4)]);
    card_generating_list.push(NUMBER_OPTIONS[rng.gen_range(0..9)]);

    let card: String = card_generating_list.iter().collect();

    return card;
}

fn generate_deck() -> Vec<String> {
    //initializing and creating deck
    let mut deck: Vec<String> = Vec::new();
    while deck.len() < INITIAL_DECK_SIZE.into() {

        //adding a single card
        deck.push(pull_card());
    }
    return deck;
}

fn does_card_match(card_1: &String, card_2: &String) -> bool {
    //check if colors match
    if card_1.chars().nth(0) == card_2.chars().nth(0) {
        return true;
    
    //check if numbers match
    } else if card_1.chars().nth(1) == card_2.chars().nth(1) {
        return true;
    }
    return false;
}

/*------------------------------*\
: starting main game
\*------------------------------*/

fn main() {
    //initializating both players decks
    let mut player1_deck = generate_deck();
    let player2_deck = generate_deck();

    let turn = 1;

    //setup the stack of cards with a random card on top
    let mut card_on_stack = pull_card();

    loop {

        /*------------------------------*\
        : plans for logic:  
        : 
        : loop infinitely:
        : print out card on stack and whos turn it is
        : tell them their deck
        : 
        : check if they have a card that matches the stack
        : if they dont then
        :   tell them they need to pull a card
        :   pull new cards until they get a card that matches
        :   
        : ask them what card they would like to play
        : 
        : check if the card matches
        : if it does 
        :   change the stack to that card
        :   remove that card from their deck
        : if it doesnt
        :   tell them it doesnt match -> pick another one
        : 
        : change to other players turn
        : 
        : repeat (but for other person)
        : 
        \*------------------------------*/

        /*------------------------------*\
        : displaying info for the player
        \*------------------------------*/

        //print out info for the player
        println!("This is the card on the stack: ");
        card_ascii(card_on_stack.as_str());

        println!("\n\nIt is player {turn}'s turn!");

        //display players deck
        println!("\nYour deck is:");
        display_player_deck(&player1_deck);

        //temporarily ending the loop so it doesnt spam w/ info
        break;
    }
}