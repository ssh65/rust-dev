/*

Write a function takes_ownership(s: String) that consumes its argument.
Call it with a String.
Try to use the original String after the call — what happens?

*/

fn main() {
    let s1 = String::from("hello");
    takes_ownership(s1);
    //println!("{}", s1); // This line would cause a compile-time error because s1 has been moved.
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
