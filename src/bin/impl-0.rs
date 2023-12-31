struct Temperature {
    degree_f: f64,
}

impl Temperature {
    fn freezing() -> Self {
        //Self refering to Temperature
        Self { degree_f: 32.0 }
    }
    fn boiling() -> Self {
        Self { degree_f: 200.12 }
    }
    fn show_temp(&self) {
        println!("{:?} degree f", self.degree_f);
    }
}

fn main() {
    let hot = Temperature { degree_f: 78.8 };
    hot.show_temp();

    let cool = Temperature::freezing();
    cool.show_temp();

    let boiling = Temperature::boiling();
    boiling.show_temp();
}
