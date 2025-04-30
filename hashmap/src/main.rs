use std::collections::HashMap;

fn main() {
    simple_hashmap();
    owner_hashmap();
    overwriting_hashmap_value();

    adding_if_isnt_present();
    updating_based_on_old_value();
}

fn simple_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score of {}: {}", team_name, score);


    for(key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn owner_hashmap() {
    // HashMap with String keys and String values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("owner_hashmap map: {:?}", &map);
}

fn overwriting_hashmap_value(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("overwriting_hashmap_value {scores:?}");
}

fn adding_if_isnt_present() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Only insert if the key is not present
    scores.entry(String::from("Yellow")).or_insert(50);
    // Only insert if the key is not present
    scores.entry(String::from("Blue")).or_insert(50);

    println!("adding_if_isnt_present {scores:?}");
}

fn updating_based_on_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
