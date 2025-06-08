/// This is comment for the function
fn main() {
    let bre = stuff();
    print!("{bre}")
}

fn stuff() -> String {
    let mut string = String::from("hi");
    string.push_str(", BROTHER CHEESE!\n");
    string
}