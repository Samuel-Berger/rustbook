fn main() {
    //Let means it is a variable.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //Const means it is a constant,
    //should also be capitalized.
    const PI:f32 = 3.14;
    println!("The value of pi is: {}", PI);

    //Shadowing, changing between immutable states?
    //Can each x can also have different types.
    let x = 3;
    let x = x + 3;
    let x = 23 + x * 7;
    println!("The value of x is: {}", x);

    data_types();
}

fn data_types() {
    //Scalar types are int, floats, booleans, chars.

    //Compound types are tuples and arrays.
    //Tuples have fixed size. But may have different types.
    let tuple : (i32, f64, char) = (500, 0.6, '😻');

    let tup = (55, 6.4, 1);

    //Pattern match on tup
    let (x, y , z) = tup;

    //Print the result.
    println!("The value of y is: {}", y);

    //Access the element of a tuple.
    //println!("The first element of tuple {:?} is: {}", tuple, tuple.0)
    println!("The first element of the tuple is: {}", tuple.0);

    //Arrays: fixed lenght and of the same type.
    let a = [1, 2, 3, 4];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("The months are: {:?}", months);

    //With types specified.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    println!("The second element is: {}", a[1]);

    //If the whole array should have the same number.
    let b = [3; 5];
    println!("{:?}", b);
    println!("The value of the last element is: {}", b[b.len()-1]);  //Might panick


}