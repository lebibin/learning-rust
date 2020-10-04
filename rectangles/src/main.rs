#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width = 30;
    let height = 50;
    let rectangle = Rectangle {
        width,
        height
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rectangle)
    );

    println!("width is {}", rectangle.width);
    println!("height is {}", rectangle.height);
    println!("rectangle is {:#?}", rectangle);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
