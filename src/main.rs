/*------------------------------------------------*\
: uno-like game
: created by: Hero
: 
: this is a MAJOR work in progress !!
: its not playable right now !!
: 
: some of this code prob isnt good im bad at rust 
\*------------------------------------------------*/

/* bringing in necessary stuff */
use std::io;

mod card_utils;
mod display_utils;

/*------------------------------*\
: starting main game
\*------------------------------*/

fn main() {
    /* initializating both players decks */
    let player1_deck = card_utils::generate_deck();
    let player2_deck = card_utils::generate_deck();

    let mut turn = 1;

    /* setup the stack of cards with a random card on top */ 
    let mut card_on_stack = card_utils::pull_card();

    /* initializing random variables */
    let mut must_pull = true;

    /*------------------------------*\
    : start main game logic loop
    \*------------------------------*/

    loop {

        /*------------------------------*\
        : game loop pseudocode:
        : 
        : set current player deck                                   ./
        :   
        : clear screen for new player                               ./
        : 
        : loop infinitely:                                          
        : print out card on stack and whos turn it is               ./
        : tell them their deck                                      ./
        : 
        : check if they have a card that matches the stack          ./
        : while they dont then                                      ./
        :   tell them they need to pull a card                      ./
        :   pull new cards until they get a card that matches       ./
        :   
        : ask them what card they would like to play                ./
        : 
        : check if they have the card                               ./
        : if they do                                                ./
        :   check if the card matches                               
        :   if it does                                              
        :       change the stack to that card                       
        :       remove that card from their deck                    
        :   if it doesnt                                            ./
        :       tell them it doesnt match -> pick another one       ./
        : if they dont have the card                                ./
        :   tell them they dont have it                             ./
        :   ask to input card                                       ./
        : 
        : check if someone has 0 cards
        : if so
        :   break out of loop and display winner
        : 
        : change to other players turn
        : 
        : repeat (but for other person)
        \*------------------------------*/

        /* clear screen (disable during development so i can see rust warnings) */
        print!("\x1B[2J");

        let mut current_player_deck: Vec<String> = Vec::new();

        if turn == 1 {
            current_player_deck = player1_deck.clone();
        } else if turn == 2 {
            current_player_deck = player2_deck.clone();
        }

        /*------------------------------*\
        : displaying info for the player
        \*------------------------------*/

        /* print out info for the player */
        println!("This is the card on the stack: ");
        display_utils::display_single_card(card_on_stack.as_str());

        println!("\n\nIt is player {turn}'s turn!");

        //display players deck
        println!("\nYour deck is:");
        display_utils::display_player_deck(&player1_deck);

        /*------------------------------*\
        : checking if they need to pull
        \*------------------------------*/

        /* check if they have a card that matches */
        for card in &current_player_deck {
            if card_utils::does_card_match(&card, &card_on_stack) {
                /* must_pull is true by default until a matching card is found */
                must_pull = false;
                break;
            }
        }

        /* pull cards until you get one that matches */
        while must_pull == true {
            /* get user to accept pulling card */
            println!("You need to pull for a card! (enter anything to accept) ");
            let mut input: String = String::new();

            io::stdin().read_line(&mut input).expect("failed to take input");
            
            /* pulling a card */
            let pulled_card = card_utils::pull_card();
            current_player_deck.push(pulled_card.clone());

            /*------------------------------------------------*/

            /* clear screen */
            print!("\x1B[2J");

            /* display stack */
            println!("This is the card on the stack: ");
            display_utils::display_single_card(card_on_stack.as_str());

            /* tell them what they pulled */
            println!("\n\nYou pulled a {pulled_card}:");
            display_utils::display_single_card(&pulled_card);

            /* display deck again */
            println!("\nHere is your new deck: ");
            display_utils::display_player_deck(&current_player_deck);

            /*------------------------------------------------*/

            /* checking if the new card matches */ 
            if card_utils::does_card_match(&pulled_card, &card_on_stack) {
                must_pull = false;

                /*------------------------------------------------*/

                /* clear screen & display info */
                print!("\x1B[2J");

                /* tell them what they pulled */
                println!("\n\nYou pulled a {pulled_card}:");
                display_utils::display_single_card(&pulled_card);

                /* tell them it matches */
                println!("You now have a card that matches!");

                /* display new info */
                println!("\nThis is the card on the stack: ");
                display_utils::display_single_card(card_on_stack.as_str());

                /* print out deck with new card */
                println!("\nHere is your new deck: ");
                display_utils::display_player_deck(&current_player_deck);

                /*------------------------------------------------*/

                break;
            }
        }

        /*------------------------------*\
        : get user to play a card
        \*------------------------------*/

        println!("Enter the card you want to play: ");
        println!("Use format: colorNumber (ex: y5)");

        /* get player input */
        let mut inputted_card: String = String::new();
        io::stdin().read_line(&mut inputted_card).expect("failed to take input");

        let inputted_card = inputted_card.trim().to_lowercase();

        let mut has_card = card_utils::does_player_have_card(&current_player_deck, &inputted_card);

        /* check if they have the card and ask again if they dont */
        while !has_card || !card_utils::does_card_match(&inputted_card, &card_on_stack) {
            /*------------------------------------------------*/

            /* clear screen & display info */
            print!("\x1B[2J");

            /* display new info */
            println!("\nThis is the card on the stack: ");
            display_utils::display_single_card(card_on_stack.as_str());

            /* print out deck with new card */
            println!("\nHere is your new deck: ");
            display_utils::display_player_deck(&current_player_deck);

            /* tell them to pick a new card */
            if !has_card {
                println!("You dont have that card!\n");
            } else {
                println!("That card doesnt match the stack...\n");
            }

            println!("Enter the card you want to play: ");
            println!("Use format: colorNumber (ex: y5)");

            /*------------------------------------------------*/

            /* get player input */
            let mut inputted_card: String = String::new();
            io::stdin().read_line(&mut inputted_card).expect("failed to take input");

            let inputted_card = inputted_card.trim().to_lowercase();
            
            has_card = card_utils::does_player_have_card(&current_player_deck, &inputted_card);

            if has_card && card_utils::does_card_match(&inputted_card, &card_on_stack) {
                println!("\nThat card matches the stack! \n");
                break; 
            }
        }

        /* update whos turn it is */
        if turn == 1 {
            turn = 2;
        } else if turn == 2 {
            turn = 1;
        }

        /* temporarily ending the loop so it doesnt spam w/ info */
        break;
    }
}
