fn main() {
    //Chapter 3 file:///home/sabe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch03-05-control-flow.html
    //f_to_c(56.0);
    
    println!("{},", fibbonaci_worker(1));
    println!("{},", fibbonaci_worker(3));
    println!("{},", fibbonaci_worker(5));
    println!("{},", fibbonaci_worker(9));
}

//Convert from Farenheit to Celsius.
fn f_to_c (tempc: f32) {
    
    let tempf = ((tempc-32.0)*5.0)/9.0;

    println!("{} degrees farenheit is {} degrees celsius.", tempc, tempf);
}


//Recursive generation of the nth Fibonacci number.
//https://benjaminbrandt.com/fibonacci-in-rust/
fn fibbonaci_worker(n: u32) -> u32{
    
    match n {
        0 => 1,
        1 => 1,
        _ => fibbonaci_worker(n-1) + fibbonaci_worker(n-2),
    }
}



/*
let a : [i32; 5];

    for element in a.iter(){
        a[element] = 2;
    }
*/
