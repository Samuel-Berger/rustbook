use std::io; //Use something not in the Prelude.
use std::cmp::Ordering;
use rand::Rng; //Use random generator imported from Cargo.

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}.", secret_number);

    loop{
    println!("Please input your guess between 1 and 100: ");

    //mut makes the variable mutable.
    //new creates a new instance of String.
    let mut guess = String::new();
    
    //& makes guess a mutalbe reference.
    io::stdin().read_line(&mut guess)
        //readline returns a io::Result, values either Ok or Err,
        //so the method .expect can be used to handle the value.
        .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse(){
            //.expect("Please type a number."); //Now correct error handling below:
            Ok(num) => num,
            Err(_) => continue,
        };



    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}
}