use std::cmp::PartialOrd;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result: &i32 = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result: &char = largest(&char_list);
    println!("The largest char is {}", result);


    let integer = Point { x: 5, y: 10 };
    println!("integer.x = {}, integer.y = {}", integer.x, integer.y);
    println!("integer.x() = {}", integer.x());
    let float = Point { x: 1.2, y: 4.3 };
    println!("float.x = {}, float.y = {}", float.x, float.y);
    println!("float.x() = {}", float.x());

    //let wont_work = Point { x: 5, y: 4.0 }; //will not work
    //not work because there is no common type for x and y

    let integer_and_float = PointMultiType { x: 5, y: 4.0 };
    println!("integer_and_float.x = {}, integer_and_float.y = {}", integer_and_float.x, integer_and_float.y);



    let p1 = PointX { x: 5, y: 10.4 };
    let p2 = PointX { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


    let integer = Option_i32::Some(5);
    println!("integer = {:?}", integer);
    let float = Option_f64::Some(5.0);
    println!("float = {:?}", float);
}

//generic function
// The function signature specifies that T must implement the PartialOrd trait
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest // or return largest;
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct PointMultiType<T, U> {
    x: T,
    y: U,
}


/* another example of generic function
enum Option<T> {
    Some(T),
    None,
}


enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

struct PointX<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointX<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointX<X2, Y2>) -> PointX<X1, Y2> {
        PointX {
            x: self.x,
            y: other.y,
        }
    }
}

#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None,
}

#[derive(Debug)]
enum Option_f64 {
    Some(f64),
    None,
}