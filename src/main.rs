use colored::*;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome!");
    let random_number = thread_rng().gen_range(1, 11);
    let mut trial_count: u8 = 0;
    loop {
        println!(
            "{}",
            "Please enter a random number  between 1 and 10:".white()
        );
        if trial_count < 2 {
            println!("Lifeline: {}", 2 - trial_count);
        }
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a valid number between 1 and 10...");
        let input: u32 = match input.trim().parse() {
            Ok(num) => {
                if num > 10 {
                    continue;
                }
                num
            }
            Err(_) => continue,
        };

        match input.cmp(&random_number) {
            Ordering::Equal => {
                println!("{}", "Congratulations, you win!".green().bold());
                break;
            }
            Ordering::Greater => {
                if trial_count >= 1 {
                    println!("{}", "You have exhausted your lifeline".red());
                    println!(
                        "{} {}",
                        "The lucky number is;".bright_cyan(),
                        random_number.to_string().bright_green()
                    );
                    break;
                }
                println!("{}", "You guessed a higher number".red());
            }
            Ordering::Less => {
                if trial_count >= 1 {
                    println!("{}", "You have exhausted your lifeline".red());
                    println!(
                        "{} {}",
                        "The lucky number is;".bright_cyan(),
                        random_number.to_string().bright_green()
                    );
                    break;
                }
                println!("{}", "You guessed a lower number".yellow());
            }
        };
        trial_count = trial_count + 1;
    }
}
