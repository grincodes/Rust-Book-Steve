
extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number");
    println!("Please input your guess");
    
    let secret_number = rand::thread_rng().gen_range(1,100);
    // :: to acceess  staic or implemented funtions ,methods or variables;
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess:u32 = guess.trim().parse().expect("Please Type A number");

    //match here is like a switch statement 
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small");
        Ordering::Greater => println!("Too Big");
        Ordering::Equal => println!("You WOn!");


    };

    println!("You guessed:  {}",guess);
}
