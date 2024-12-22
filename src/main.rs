use crate::solutions::Problem171;

mod solutions;

fn main() {
    println!("abc {}", Problem171::title_to_number("A".to_string()));
    println!("abc {}", Problem171::title_to_number("ZY".to_string()));
}
