/* Write code where you create a String, then:
Move it into another variable.
Clone it into another variable.
Show how the moved one becomes invalid, but the cloned one still works. */

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2 
    let s3 = s2.clone(); // s2 is cloned to s3  
    println!("{} {}", s2, s3); // both s2 and s3 are valid 
}
