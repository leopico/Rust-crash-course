struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_quantity(item: &GroceryItem) {
    //& is borrowing or refrencing from ownership
    println!("quantity: {:?}", item.quantity);
}

fn display_id(item: &GroceryItem) {
    //& is borrowing or refrencing from ownership
    println!("id: {:?}", item.id);
}

fn main() {
    let my_item = GroceryItem { quantity: 3, id: 5 };
    display_quantity(&my_item); //& is borrowing or refrencing from ownership
    display_id(&my_item); //& is borrowing or refrencing from ownership
}
