use std::io;

use cards::hand::Hand;
use cards::deck::Deck;

fn main() -> io::Result<()> {
    let mut deck = Deck::new();
    let mut hand = Hand::empty();
    let mut other_hand = Hand::empty();

    deck.shuffle();

    for _ in 0..5 {
        deck.deal(&mut hand);
        deck.deal(&mut other_hand);
    }

    println!("Here is your starting hand: {}", hand);
    println!("Would you like to change any cards?");

    let mut input = String::new();
    let mut index_to_change: Vec<usize> = vec![];

    println!("Type the index of the cards you want to change, separated by a comma if many:");
    loop {
        if io::stdin().read_line(&mut input).is_err() { panic!("Error while reading stdin input!"); }

        // C^D
        if input.pop().is_none() {
            println!();
            continue;
        }

        // Enter (no cards to change)
        if input.is_empty() {
            break;
        }

        let mut valid = true;
        for nstr in input.split(',') {
            if let Ok(n) = nstr.parse::<usize>() {
                if n < 5 && !index_to_change.contains(&n) {
                    index_to_change.push(n);
                } else {
                    valid = false;
                    break;
                }
            }
        }
        if index_to_change.len() <= 5 && valid {
            break;
        }
        println!("Wrong format or number of indexes! Try again: ");
        index_to_change.clear();
    }

    let nums = index_to_change.len();
    index_to_change.sort_by(|a, b| b.cmp(a));
    for i in index_to_change {
        hand.remove(&i);
    }

    for _ in 0..nums {
        deck.deal(&mut hand);
    }

    let my_comb = hand.find_highest_comb();
    let their_comb = other_hand.find_highest_comb();

    println!("You: {} => {}", hand, my_comb);
    println!(" Me: {} => {}", other_hand, their_comb);

    if my_comb > their_comb {
        println!("You win! Congrats!");
    } else if my_comb < their_comb {
        println!("I win! Haha!");
    } else {
        println!("It's a draw... Nobody wins!");
    }

    Ok(())
}
