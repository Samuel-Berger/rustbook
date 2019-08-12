use std::io; //Use something not in the Prelude.

fn main() {
    println!("Guess the number");
    println!("Please input your guess: ");

    //mut makes the variable mutable.
    //new creates a new instance of String.
    let mut guess = String::new();
    
    //& makes guess a mutalbe reference.
    io::stdin().read_line(&mut guess)
        //readline returns a io::Result, values either Ok or Err,
        //so the method .expect can be used to handle the value.
        .expect("Failed to read line.");

        println!("You guessed: {}", guess);

}