fn print_it(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let owned_string = "owned string".to_owned();
    let another_string = String::from("another");
    println!("{:?}", &owned_string);
    println!("{:?}", &another_string);

    print_it("a string slice");
}
