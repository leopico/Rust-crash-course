struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "leopico".to_owned(),
            count: 3,
        },
        LineItem {
            name: String::from("leo"),
            count: 4,
        },
    ];

    for item in receipt {
        print_name(&item.name);
        println!("count: {:?}", item.count);
    }
}
