/*------------------------------------------------------*\
: uno-like game
: created by: Hero
: 
: This is a work in progress !!
: I will add many more features to this later
: 
: Yes, I know this code is generally pretty bad and messy 
\*------------------------------------------------------*/

/* bringing in necessary stuff */
use std::io;

mod card_utils;
mod display_utils;

/*------------------------------*\
: starting main game
\*------------------------------*/

fn main() {
    /* initializating game control variables & decks */
    let mut player1_deck = card_utils::generate_deck();
    let mut player2_deck = card_utils::generate_deck();

    let mut turn = 1;

    let mut must_pull = true;

    /* setup the stack of cards with a random card on top */ 
    let mut card_on_stack = card_utils::pull_card();

    /*------------------------------*\
    : Display HTP before game starts
    \*------------------------------*/

    /* clear screen */
    print!("\x1B[2J");

    display_utils::display_how_to_play();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("failed to take input");

    /*------------------------------*\
    : start main game logic loop
    \*------------------------------*/

    loop {
        /* clear screen */
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

        println!("This is the card on the stack: ");
        display_utils::display_single_card(card_on_stack.as_str());

        println!("\n\nIt is player {turn}'s turn!");

        println!("\nYour deck is:");
        display_utils::display_player_deck(&current_player_deck);

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

            display_utils::display_general_info(&pulled_card, &card_on_stack, &current_player_deck);

            /* checking if the new card matches */ 
            if card_utils::does_card_match(&pulled_card, &card_on_stack) {
                must_pull = false;

                display_utils::display_general_info(&pulled_card, &card_on_stack, &current_player_deck);

                /* tell them it matches */
                println!("\nYou now have a card that matches!\n");

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

        inputted_card = inputted_card.trim().to_lowercase();

        let mut has_card = current_player_deck.contains(&inputted_card);

        /* check if they have the card and ask again if they dont */
        while !has_card || !card_utils::does_card_match(&inputted_card, &card_on_stack) {
            /* clear screen & display new info */
            print!("\x1B[2J");

            println!("\nThis is the card on the stack: ");
            display_utils::display_single_card(card_on_stack.as_str());

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

            /* get player card choice */
            inputted_card.clear();
            io::stdin().read_line(&mut inputted_card).expect("failed to take input");

            inputted_card = inputted_card.trim().to_lowercase();
            
            /* check if player has card */
            has_card = current_player_deck.contains(&inputted_card);

            /* display message and break if they have a good card */
            if has_card && card_utils::does_card_match(&inputted_card, &card_on_stack) {
                println!("\nThat card matches the stack! \n");
                break; 
            }
        }

        /* update stack and player deck */
        card_on_stack = inputted_card.clone();
        card_utils::remove_card_from_deck(&mut current_player_deck, &card_on_stack);

        /*------------------------------*\
        : display info at end of turn
        \*------------------------------*/

        /* clear screen */
        print!("\x1B[2J");

        /* end game if someone has 0 cards */
        if current_player_deck.len() == 0 {
            println!("Congratulations, Player {turn} has won! \n");
            break;
        }

        /* update to next players turn */
        println!("It is now player {turn}'s turn! (enter anything to accept) ");
        let mut input: String = String::new();

        io::stdin().read_line(&mut input).expect("failed to take input");

        if turn == 1 {
            player1_deck = current_player_deck;
            turn = 2;
        } else if turn == 2 {
            player2_deck = current_player_deck;
            turn = 1;
        }
        must_pull = true;
    }
}
