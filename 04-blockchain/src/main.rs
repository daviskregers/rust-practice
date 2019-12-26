#[macro_use]
extern crate serde_derive;

mod blockchain;

use std::io;
use std::io::Write;

use std::process;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush().expect("Something went wrong");
    io::stdin().read_line(&mut miner_addr).expect("Something went wrong");

    print!("Difficulty: ");
    io::stdout().flush().expect("Something went wrong");
    io::stdin().read_line(&mut difficulty).expect("Something went wrong");
    let diff = difficulty.trim().parse::<u32>().expect("We need an integer");

    println!("Generating genesis block!");

    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice:  ");
        io::stdout().flush().expect("Something went wrong");
        choice.clear();
        io::stdin().read_line(&mut choice).expect("Something went wrong");
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting!");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("enter sender address:  ");
                io::stdout().flush().expect("Something went wrong");
                io::stdin().read_line(&mut sender).expect("Something went wrong");
                print!("enter receiver address:  ");
                io::stdout().flush().expect("Something went wrong");
                io::stdin().read_line(&mut receiver).expect("Something went wrong");
                print!("Enter amount:  ");
                io::stdout().flush().expect("Something went wrong");
                io::stdin().read_line(&mut amount).expect("Something went wrong");

                let res = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount.trim().parse().unwrap());

                match res {
                    true => println!("Transaction added!"),
                    false => println!("Transaction failed!"),
                }
            },
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }, 
            3 => {
                let mut new_diff = String::new();
                print!("Enter new difficulty:  ");
                io::stdout().flush().expect("Something went wrong");
                io::stdin().read_line(&mut new_diff).expect("Something went wrong");
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated difficulty!"),
                    false => println!("Failed to update difficulty!")
                }
            }, 
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward:  ");
                io::stdout().flush().expect("Something went wrong");
                io::stdin().read_line(&mut new_reward).expect("Something went wrong");
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed to update reward"),
                }
            },
            _ => println!("\tinvalid option please retry \t"),
        }
    }
}
