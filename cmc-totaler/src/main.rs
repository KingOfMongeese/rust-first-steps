// totals all the cmc for a mtg deck
// pass highest cmc card as only cmdline arg example `cargo run -- 5` if the highest cmc in the deck is 5.

use std::env;
use std::io::stdin;

/// returns the highest cmc given by the user
fn get_args() -> i32 {
    // get the args
    let args: Vec<String> = env::args().collect();

    // check if we have args
    if args.len() < 2 {
        eprintln!("Usage ERROR an int is required to represent the highest CMC value.");
        std::process::exit(1);
    } else {
        // parse and ensure valid int
        let parsed_value = args[1].parse::<i32>();
        match parsed_value {
            Ok(value) => value,
            Err(value) => {
                eprintln!("Usage ERROR {} is not a number.", value);
                std::process::exit(2);
            }
        }
    }
}

fn main() {
    let highest_cmc = get_args();
    let mut total: i32 = 0;
    println!("Highest CMC is {}", highest_cmc);

    // loop through each cmc
    for cmc in 0..=highest_cmc {
        println!("Enter the number of {} CMC cards>", cmc);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");

        input = input.trim().to_string();
        if !input.is_empty() {
            // parse to i32
            let parsed_input = input.parse::<i32>();
            match parsed_input {
                Ok(value) => {
                    total += value * cmc;
                }
                Err(_value) => eprintln!("{}", _value),
            }
        }
    }

    let cards_in_deck = 100.0; // default to cmder deck for now
    let cmc_average = total as f32 / cards_in_deck;

    println!("\nTotal CMC count: {}", total);
    println!("CMC Average: {}", cmc_average);
}
