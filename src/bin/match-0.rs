/*
Same as if, else if but match will check on compile time
*/

fn main() {
    let my_name = "Bill";
    match my_name {
        "Jayson" => println!("that is my name"),
        "Bob" => println!("not my name"),
        "Alice" => println!("hello alice"),
        _ => println!("nice to meet you"),
    }
}
