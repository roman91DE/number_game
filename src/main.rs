use std::io;
use rand::Rng;
use std::cmp::Ordering;



// A simple number guessing game in rust based on https://doc.rust-lang.org/book


fn main() {
    
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a Number between 1 and 100!");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading Stdin!");

        let input: u32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("Input was {input}");


        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }   
        }
    }
}
