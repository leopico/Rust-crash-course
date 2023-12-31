fn main() {
    let coord = (2, 3);
    println!("{:?} {:?}", coord.0, coord.1);

    let (x, y) = (3, 4);
    println!("{:?} {:?}", x, y);

    let (name, age) = ("leo", 31);
    println!("{:?} {:?}", name, age);

    let favorites = ("red", 14, "TX", "pizza", "TV SHOW", "home");
    let state = favorites.2;
    let place = favorites.5;
    println!("My {:?} at {:?}", place, state);
}
