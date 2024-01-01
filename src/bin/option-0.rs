struct Customer {
    age: Option<i32>,
    email: String,
}

fn main() {
    let mark = Customer {
        age: Some(22),
        email: "mark@gmail.com".to_owned(),
    };
    let leopico = Customer {
        age: None,
        email: "leopico@gmail.com".to_owned(),
    };

    match mark.age {
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("customer age not provided"),
    }

    match leopico.age {
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("customer age not provided"),
    }
}
