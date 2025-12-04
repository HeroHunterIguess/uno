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

//pulling a single card
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


//generating a deck
fn generate_deck() -> Vec<String> {
    //initializing and creating deck
    let mut deck: Vec<String> = Vec::new();
    while deck.len() < 7 {
        //adding a single card
        deck.push(pull_card());
    }
    return deck;
}

/*------------------------------*\
: starting main game
\*------------------------------*/

fn main() {
    //initializating both players decks
    let player1_deck = generate_deck();
    let player2_deck = generate_deck();

    println!("player 1s deck: {:?}",player1_deck);
    println!("player 2s deck: {:?}",player2_deck);

    //setup the stack of cards with a random card on top
    let card_on_stack = pull_card();
    println!("{card_on_stack}");

}
