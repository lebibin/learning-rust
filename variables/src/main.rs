fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is {}", x);

    let x = "shadowed";
    println!("The value of x is {}", x);

    let allowed = false;
    println!("You're allowed : {}", allowed);

    let arr: [i32; 5] = [1,2,3,4,5];
    println!("Array[0]: {}", arr[0]);
    // panic!
    // println!("Array[10]: {}", arr[10]);
}
