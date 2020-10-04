fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area((width, height))
    );
}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
