// Create a String, assign it to another variable, and then try to print the old variable.

fn main() { 
    let s1 = String::from("hello"); 
    let s2 = s1; // s1 is moved to s2 
    // println!("{}", s1); // This line would cause a compile-time error because s1 is no longer valid 
    println!("{}", s2); // This works fine 
}