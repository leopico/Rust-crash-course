struct Employee {
    name: String,
}

fn main() {
    let emp_name = "Leopico".to_owned();
    let emp_nam_from = String::from("Leo");
    let emp = Employee { name: emp_name };
    let emp_from = Employee { name: emp_nam_from };
}
