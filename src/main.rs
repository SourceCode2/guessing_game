// Made by Davide Sofronia, https://github.com/SourceCode2/guessing_game
extern crate rand;
extern crate colored;

use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("{}", "Guess the number!".red().bold());
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    loop {
        print!("{}","Please input a guess: ".green());
        io::stdout().flush().unwrap();
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read input");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        }; 

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("{}", "You win!".green().bold());
                break;
            },
        }
    }
}
