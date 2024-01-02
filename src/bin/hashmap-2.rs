use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();

    stock.insert("Chair", 5);
    stock.insert("Bed", 5);
    stock.insert("Table", 5);
    stock.insert("Couch", 0);

    let mut total_stock = 0;

    for (item, qty) in stock.iter() {
        total_stock = total_stock + qty;
        let stock_count = if qty == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", qty)
        };

        println!("item = {:?}, stock = {:?}", item, stock_count);
    }

    println!("total_stock = {:?}", total_stock);
}
