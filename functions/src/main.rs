use std::f32::consts;   //For value of Pi.

fn main() {
    foo(34.0,33.0);

    let c  = circumference(5.0);
    println!("The circumference is: {}.", c);

    bar("String: apa");
    
    let apa = heba("monkey: &str");
    println!("{}", apa);

}

fn foo(radius: f32, diameter: f32) {
    let result = radius + diameter;
}

/*
The return value of the function is synonymous with the value
of the final expression in the block of the body of a function.
*/
fn circumference(diameter: f32) -> f32 {
    //No let before evaluation.
    //No semicololon after diameter when return thingy.
    consts::PI * diameter
}

//http://xion.io/post/code/rust-string-args.html
fn bar(apa: &str) {
    println!("{}", apa);
}

//https://stackoverflow.com/a/43080280
fn heba(monkey: &str) -> &str {
    monkey
}