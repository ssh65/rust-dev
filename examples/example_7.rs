fn main() {
    let mut s1 = String::from("hello");
    let r1 = &s1; // no problem
    let r2 = &s1; // no problem
    println!("{} and {}", r1, r2);
    let r3 = &mut s1;
    println!("{}", r3);
}
