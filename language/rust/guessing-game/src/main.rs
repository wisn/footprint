extern crate rand;

use std::cmp::Ordering;
use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    println!("A Guessing Game!");
    println!("Guess the number and let see how much steps would you get?");

    let mut step = 1;
    let number = rand::thread_rng().gen_range(-1000, 1000);

    loop {
        println!();
        print!("Guess the number: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");

        if guess.trim() == "exit" {
            break;
        }

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Yas! You win!\n");
                println!("Took you {} times.", step);
                break;
            },
        }

        step += 1;
    }
}

