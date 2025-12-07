/*------------------------------------------------*\
: uno-like game
: created by: Hero
: 
: some of this code prob isnt good im bad at rust 
\*------------------------------------------------*/

use rand::Rng;

//setting up constants for deck generation
const COLOR_OPTIONS: [char; 4] = ['r','y','b','g'];
const NUMBER_OPTIONS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

/*------------------------------*\
: defining essential functions   
\*------------------------------*/

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
\\-------/");
}

fn display_player_deck(deck: &Vec<String>) {
    let mut card_rows = deck.len().div_ceil(5);
    let mut cards_left = deck.len();
    //loop and display ascii for every card in the deck
    while card_rows >= 0 && cards_left >= 1{
        
        
        
        cards_left -= 1;
    }
}

fn pull_card() -> String {
    //initialize rng and list the card gets made in
    let mut rng = rand::thread_rng();
    let mut card_generating_list: Vec<char> = Vec::new();

    //generating the card
    card_generating_list.push(COLOR_OPTIONS[rng.gen_range(0..4)]);
    card_generating_list.push(NUMBER_OPTIONS[rng.gen_range(0..8)]);

    let card: String = card_generating_list.iter().collect();

    return card;
}

fn generate_deck() -> Vec<String> {
    //initializing and creating deck
    let mut deck: Vec<String> = Vec::new();
    while deck.len() < 7 {
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
    let player1_deck = generate_deck();
    let player2_deck = generate_deck();

    let turn = 1;

    //setup the stack of cards with a random card on top
    let card_on_stack = pull_card();

    loop {

        //print out info for the player
        println!("This is the card on the stack: ");
        card_ascii(card_on_stack.as_str());

        println!("\n\nIt is player {turn}'s turn!");
        println!("\nYour deck is: \n{:?}",player1_deck);

        display_player_deck(&player1_deck);
        
        break;

    }
}
