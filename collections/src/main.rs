fn main() {
    
    vector();
    vector2();
    vector3();
    vector4();

    vector_push();

    vector_iterating();
    vector_add50();

    vector_multiple_types();
}

fn vector() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);
}

fn vector2(){
    let x = vec![1, 2, 3];
    println!("{:?}", x);
}

fn vector3() {
    let y = vec![1, 2, 3, 4, 5];

    let third: &i32 = &y[2];
    println!("The third element is {third}");

    let third: Option<&i32> = y.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn vector4() {
    let v = vec![1, 2, 3, 4, 5];

    //let _does_not_exist = &v[100];
    let _does_not_exist = v.get(100);
}

fn vector_push() {
    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    //v.push(6); error: cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("The first element is: {first}");
}

fn vector_iterating() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}

fn vector_add50(){
    let mut v = vec![100, 32, 57, 30];
    println!("removed from v, index 2 {:?}", v.remove(2));
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_multiple_types() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    print!("{:?}", row);
}
