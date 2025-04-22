fn main() {

    let _ = Coin::Nickel;
    let _ = Coin::Dime;
    let _ = Coin::Quarter;
    value_in_cents(Coin::Penny);



    let five = Some(5);
    println!("five: {:?}", five);
    let six = plus_one(five);
    println!("six: {:?}", six);
    let none = plus_one(None);
    println!("none: {:?}", none);


    roll_dice();
    roll_dice2();
    roll_dice3();
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}



fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn roll_dice(){
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn add_fancy_hat() {
    println!("Add fancy hat");
}
fn remove_fancy_hat() {
    println!("Remove fancy hat");
}
fn move_player(num_spaces: u8) {
    println!("Move player {} spaces", num_spaces);
}
fn reroll() {
    println!("Reroll");
}

fn roll_dice2() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }    
}


fn roll_dice3() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}