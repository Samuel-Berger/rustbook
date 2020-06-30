//file:///home/ghwomb/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch03-05-control-flow.html#summary

fn main() {
    //Chapter 3 file:///home/sabe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch03-05-control-flow.html
    f_to_c(56.0);
    c_to_f(23.0);

    fib_array(12);
    fib_array(3);

    /*
    println!("{},", fibbonaci_worker(0));
    println!("{},", fibbonaci_worker(1));
    println!("{},", fibbonaci_worker(2));
    println!("{},", fibbonaci_worker(3));
    println!("{},", fibbonaci_worker(4));
    */
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

