mod rectangle;
use rectangle::Rectangle;

fn main() {
    no_struct();
    with_tuples();
    with_struct();
}

fn no_struct() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_no_struct(width1, height1)
    );
}

fn area_no_struct(width: u32, height: u32) -> u32 {
    width * height
}

fn with_tuples() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_tuples(rect1)
    );
}

fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


fn with_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_struct(&rect1)
    );

    dbg!(&rect1);
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}