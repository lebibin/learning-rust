fn main() {
    let num = 5;
    let x = if num < 5 {
        println!("Less than 5!");
        5
    } else {
        println!("Equal or greater than 5!");
        10
    };
    println!("x is {}", x);
}
