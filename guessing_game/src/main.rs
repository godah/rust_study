use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number;");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line( &mut guess)
            .expect("Failed to read line");
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Your guessed: {}", guess);
        //println!("Your guessed: {guess}"); //same as above        


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                print!("You win!");
                break;
            }
        }
    }
    /* 
    print!("Another input form:");
    let mut guess1 = String::new();
    match io::stdin().read_line(&mut guess1) {
        Ok(n) => {
            println!("{n} bytes read");
            print!("{guess1}");
        }
        Err(error) => print!("Error: {error}")
    }
    */
}
