use std::collections::HashMap;

//hashmap stored information as key and value
fn main() {
    let mut people = HashMap::new();
    people.insert("Susan", 21);
    people.insert("leopico", 31);
    people.insert("leo", 21);
    people.insert("MyaMya", 51);

    match people.get("Leopico") {
        Some(age) => println!("age = {:?}", age),
        None => println!("not found"),
    }
}
