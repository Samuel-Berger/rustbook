use std::io;            //Use something not in the Prelude.
use std::cmp::Ordering;
use rand::Rng;          //Use random generator imported from Cargo.
//use std::error::Error;  //https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guesses = 5;

    //println!("The secret number is: {}.", secret_number);

    while guesses > 0 {
        println!("Please input your guess between 1 and 100: ");

        //mut makes the variable mutable.
        //new creates a new instance of String.
        let mut guess = String::new();
        
        //& makes guess a mutalbe reference.
        io::stdin().read_line(&mut guess)
        //readline returns a io::Result, values either Ok or Err,
        //so the method .expect can be used to handle the value.
        .expect("Failed to read line.");
            
        //This is a shadow value of the former variable guess.
        let guess: u32 = match guess.trim().parse(){
            //.expect("Please type a number."); //Now correct error handling below:
            Ok(num) => num,
            //Continue: go to the next iteration of the loop.
            Err(_) => continue,
        };

        // Match expression made of arms. This example has three arms.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
              println!("You win!");
              break;
            }
        }
        guesses = guesses - 1;
        println!("You have {} guesses left.", guesses);
    }
}