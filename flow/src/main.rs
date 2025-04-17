use std::io;
use std::cmp::Ordering;

fn main() {
    let mut number = String::new();
    println!("Please input a number.");
    
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    

    let number: i32 =  number
        .trim()
        .parse() 
        .expect("Please type a number!");

    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is equal to 5");
    } else {
        println!("The number is greater than 5");
    }

    let number_key = 5;
    match number.cmp(&number_key) {
        Ordering::Less => println!("The number is less than 5"),
        Ordering::Greater => println!("The number is greater than 5"),
        Ordering::Equal => println!("The number is equal to 5"),
    }
   
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    disambiguate_between_multiple_loops();
    while_sample();
    for_sample();
    for_range_sample();
}

fn disambiguate_between_multiple_loops(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_sample() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_sample() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    
}

fn for_range_sample() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}