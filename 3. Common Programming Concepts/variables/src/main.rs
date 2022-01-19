use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // constant
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // let x = 5;
    //
    // let x = x + 1;
    //
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }
    // println!("The value of x is: {}", x);

    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("{}", guess);

    //@ Data types

    let a = [1,2,3,4,5];

    println!("Enter array index");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index:usize = index
        .trim()
        .parse()
        .expect("Index is not a number");

    let element = a[index];

    println!("The value is {}", element);
}

