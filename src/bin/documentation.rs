///A favorite color
enum Color {
    Red,
    Blue,
}

///A piece of mail.
struct Mail {
    ///The destination address.
    address: String,
}

///Adds tow numbers together.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let num = add(2, 3);
    println!("add {:?}", num);
}
