fn main() {
    let number = 3;

    if number != 0 {
        println!("The number was {}.", number);
    }


    let condition = false;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    loop {
        println!("!");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    let a : [i32; 5];

    for element in a.iter(){
        a[element] = 2;
    }
}
