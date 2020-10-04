#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.area() >= rectangle.area()
    }
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
        rectangle.area()
    );

    println!("width is {}", rectangle.width);
    println!("height is {}", rectangle.height);
    println!("rectangle is {:#?}", rectangle);

    let rectangle2 = Rectangle {
        width: 10,
        height: 40
    };

    let rectangle3 = Rectangle {
        width: 60,
        height: 45
    };

    let square = Rectangle::square(30);

    println!("Rectangle#1 can hold Rectangle#2: {}", rectangle.can_hold(&rectangle2));
    println!("Rectangle#1 can hold Rectangle#3: {}", rectangle.can_hold(&rectangle3));
    println!("Rectangle#1 can hold Square: {}", rectangle.can_hold(&square));
}

