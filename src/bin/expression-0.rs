enum Menu {
    Burger,
    Fries,
    Drink,
}

fn main() {
    let my_num = 3;
    let is_lt_5 = if my_num < 5 { true } else { false };
    // let is_lt_5 = my_num < 5;

    let my_num = 5;
    let message = match my_num {
        1 => "hello",
        _ => "goodbye",
    };

    let paid = true;
    let item = Menu::Drink;
    let drink_type = "water";
    let order_placed = match item {
        Menu::Drink => {
            if drink_type == "water" {
                true
            } else {
                false
            }
        }
        _ => true,
    };
}
