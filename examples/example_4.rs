/*

Write a function calculate_length(s: &String) -> usize that:
Takes a reference to a string.
Returns its length.
Show that you can still use the original string after calling it.

*/

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("String is {} and its length is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
