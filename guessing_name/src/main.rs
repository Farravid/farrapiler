use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!\nPlease input your guess");

    loop {
        let mut guess : String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let secret_number = rand::thread_rng().gen_range(0..=5);
        println!("Guess secret number: {}", secret_number);

        let guess : u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a number!");
                    continue;
                },
            };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
             }
            Ordering::Less => println!("Number too small"),
            Ordering::Greater => println!("Number too big"),
        }
    }
}
