#![allow(dead_code)]

fn main() {

    /*
    f_to_c(56.0);
    c_to_f(23.0);

    fib_array(12);
    fib_array(3);
    */

    twelve_days_of_christmas();
}

//Convert from Farenheit to Celsius.
fn f_to_c (tempf: f32) {
    
    let tempc = (tempf-32.0)/1.8;

    println!("{} degrees farenheit is {} degrees celsius.", tempf, tempc);
}

//Convert from Celsius to Farenheit.
fn c_to_f (tempc: f32) {

    let tempf = tempc * 1.8 + 32.0;

    println!("{} degrees celsius is {} degrees farenheit.", tempc, tempf);

}

fn fib_array(end_number: u32){

    let mut answers: Vec<u32> = Vec::new();
    let mut counter = 0;

    while counter != end_number {
        answers.push( fibbonaci_worker(counter));
        //println!("{},", fibbonaci_worker(counter));
        counter = counter + 1;
    }

    for i in &answers {
        println!("{},", i);
    }

}

//Recursive generation of the nth Fibonacci number.
//https://benjaminbrandt.com/fibonacci-in-rust/
fn fibbonaci_worker(n: u32) -> u32{
    
    match n {
        0 => 0,
        1 => 1,
        _ => fibbonaci_worker(n-1) + fibbonaci_worker(n-2),
    }
}


//Can be re-written with tuple instead of two arrays.
fn twelve_days_of_christmas(){
    
    let gifts = [
        "And a partridge in a pear tree.",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let mut day = 1;

    while day < 13 {

        println!("On the {} day of Christmas", days[day-1]);
        println!("My true love gave to me");
        
        if day == 1{
            println!("A partridge in a pear tree.");
        }
        
        else{
            let mut i = day;
            while i > 0 {
                println!("{}", gifts[i-1]);
                i = i - 1;
            }
        }   
        
        println!();
        day = day + 1;
    }
}