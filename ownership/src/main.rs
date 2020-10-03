fn main() {
    // let x = String::from("WTF");
    // let y = x;
    // println!("x is {}", x);
    let s = String::from("hello");
    let t = takes_ownership(s);

    // println!("s is {}", s);
    println!("t is {}", t);

    let i = 8;
    makes_copy(i);
}

fn takes_ownership(str: String) -> String {
    println!("takes ownership: {}", str);
    str
}

fn makes_copy(int: i32) {
    println!("makes copy: {}", int);
}
