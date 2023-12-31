enum Color {
    Brown,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Brown"),
            Color::Red => println!("Red"),
        }
    }
}

struct Dimension {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimension {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimension: Dimension,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimension: Dimension) -> Self {
        Self {
            color,
            weight,
            dimension,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimension.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimension = Dimension {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let large_dimension = Dimension {
        width: 3.0,
        height: 4.0,
        depth: 5.0,
    };
    let small_box = ShippingBox::new(5.0, Color::Brown, small_dimension);
    let large_box = ShippingBox::new(5.0, Color::Red, large_dimension);
    small_box.print();
    large_box.print();
}
