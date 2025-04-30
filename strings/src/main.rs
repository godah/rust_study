fn main() {
    
    let mut s = String::new();
    println!("Please enter a string:");
    std::io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    let s = s.trim();
    println!("You entered: {}", s);

    let data = "initial contents";

    let s1 = data.to_string();
    println!("s1: {}", s1);
    // the method also works on a literal directly:
    let s3 = "initial contents".to_string();
    println!("s3: {}", s3);

    utf8_strings();
    update_string();
    concat_string(); 
    push_str();
    plus_str();
    tic_tac_toe();
    tic_tac_toe2();
    
    iterate_string();
    iterate_string_bytes();
    println!("get_char_from_string: {}", get_char_from_string(&s1, 5));
}


fn utf8_strings(){
    println!("{}", String::from("السلام عليكم"));
    println!("{}", String::from("Dobrý den"));
    println!("{}", String::from("Hello"));
    println!("{}", String::from("שלום"));
    println!("{}", String::from("नमस्ते"));
    println!("{}", String::from("こんにちは"));
    println!("{}", String::from("안녕하세요"));
    println!("{}", String::from("你好"));
    println!("{}", String::from("Olá"));
    println!("{}", String::from("Здравствуйте"));
    println!("{}", String::from("Hola"));
}

fn update_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("update_string: s is {s}");
}

fn concat_string() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("concat_string: s2 is {s2}");
}


fn push_str() {
    let mut s1 = String::from("fo");
    s1.push('o');
    println!("push_str: s1 is {s1}");
}


fn plus_str() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("plus_str: s3 is {s3}");
}


fn tic_tac_toe() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("tic_tac_toe: {}", s);
}

fn tic_tac_toe2() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("tic_tac_toe2: {}", s);
}


fn iterate_string() {   
    for c in "안녕하세요".chars() {
        println!("{c}");
    }
}

fn iterate_string_bytes() {   
    let bytes = "안녕하세요".bytes();
    println!("iterate_string_bytes {:?}", bytes);
}


fn get_char_from_string(string: &String, index: usize) -> String{
    string.chars().nth(index).unwrap_or(' ').to_string()
}

