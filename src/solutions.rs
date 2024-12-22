pub struct Problem171;
impl Problem171 {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result: i32 = 0;
        // for c in column_title.chars().rev() {
        for c in column_title.chars() {
            // result += ((c.to_ascii_uppercase() as i32 - 64) * 26_i32.pow(order));
            result = result * 26 + (c.to_ascii_uppercase() as i32 - 64);
        }
        result
    }
}
