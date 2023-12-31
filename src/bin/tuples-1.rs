//function tuples
fn coordinate() -> (i32, i32) {
    (1, 4)
}

fn main() {
    let (x, y) = coordinate();

    if y > 5 {
        println!(">5");
    } else if y < 5 {
        println!("<5");
    } else {
        println!("equal 5");
    }
}
