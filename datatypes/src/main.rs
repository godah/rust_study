use std::io;

fn main() {
    let a = [1,2,3,4,5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Fail to read line");

    let index: usize = index
                        .trim()
                        .parse()
                        .expect("Index entered was not a number");
    
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    let b = [3;5];
    println!("The value of b {:?}", b);

    let c : [i32; 5] = [1,2,3,4,5];
    println!("The value of c {:?}", c);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("The value of months {:?}", months);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of five_hundred {}", five_hundred);
    println!("The value of six_point_four {}", six_point_four);
    println!("The value of one {}", one);
}
