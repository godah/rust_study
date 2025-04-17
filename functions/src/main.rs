fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_params(5, 10);
    println!("{}", another_function_with_params_and_return(5, 10));
    println!("{}", another_function_with_params_and_return_2(5, 10));
    println!("{}", another_function_with_params_and_return_3(5, 10));
    println!("{}", another_function_with_params_and_return_4(5, 10));

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_params(x: i32, y: i32) {
    println!("The sum of {} and {} is {}", x, y, x + y);
}
fn another_function_with_params_and_return(x: i32, y: i32) -> i32 {
    x + y
}
fn another_function_with_params_and_return_2(x: i32, y: i32) -> i32 {
    return x + y;
}
fn another_function_with_params_and_return_3(x: i32, y: i32) -> i32 {
    let sum = x + y;
    sum
}
fn another_function_with_params_and_return_4(x: i32, y: i32) -> i32 {
    let sum = x + y;
    return sum;
}