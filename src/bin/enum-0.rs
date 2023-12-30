enum Direciton {
    Left,
    Right,
    Up,
}

fn main() {
    let go_right = Direciton::Right;
    match go_right {
        Direciton::Left => println!("Go to left"),
        Direciton::Right => println!("Go to right"),
        Direciton::Up => println!("Go to up"),
    };
}
