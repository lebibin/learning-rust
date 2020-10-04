struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let user = User {
        email: String::from("kevdashdev+rust@gmail.com"),
        username: String::from("kevin"),
        sign_in_count: 0,
        active: false
    };

    // println!("user #1: {}", user);
}
