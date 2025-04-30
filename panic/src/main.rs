use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::net::IpAddr;

fn main() {
    //panic!("This is a panic message!");

    let v = vec![1, 2, 3];
    // This will cause a panic because we are trying to access an index that is out of bounds
    //let _x = v[99];

    
    
    //open_file_simple_error();
    //open_file_different_errors();
    //alternative_to_match();
    //unrap();
    //expected();
    //let _ = read_username_from_file();

    //let read_username_from_file_short = read_username_from_file_short();
    //let read_username_from_file_short = read_username_from_file_small();
    let read_username_from_file_short = read_username_from_file_by_fs();
    match read_username_from_file_short {
        Ok(username) => println!("Username: {}", username),
        Err(e) => panic!("Problem reading the file: {:?}", e),
    }
   
}

fn open_file_simple_error(){
    let greeting_file_result = File::open("greeting.txt");
    match greeting_file_result {
        Ok(file) => {
            // File opened successfully, do something with it
            println!("File opened successfully: {:?}", file);
        }
        Err(e) => panic!("Failed to open file: {}", e),
    }
}

fn open_file_different_errors() {
    let greeting_file_result = File::open("greeting.txt");
    match greeting_file_result {
        Ok(file) => {
            // File opened successfully, do something with it
            println!("File opened successfully: {:?}", file);
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                panic!("File not found: {}", e);
            }
            ErrorKind::PermissionDenied => {
                panic!("Permission denied: {}", e);
            }
            _ => {
                panic!("Failed to open file: {}", e);
            }
        },
    }
}

fn alternative_to_match(){
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn unrap(){
    let greeting_file = File::open("greeting.txt").unwrap();
    // File opened successfully, do something with it
    println!("File opened successfully: {:?}", greeting_file);
}

fn expected() {
    let _greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


fn read_username_from_file_small() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_by_fs() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn _expect_exemple() {
    let _home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}