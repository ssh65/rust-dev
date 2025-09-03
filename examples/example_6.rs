fn main() {
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    // let s3 = &mut s1; // This line will cause a compile-time error
    println!("{}", s2);
}
