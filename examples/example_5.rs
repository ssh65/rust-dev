fn main() {
    let mut s1 = String::from("hello");
    append_world(&mut s1);
    println!("{}", s1);
}

fn append_world(s: &mut String) {
    s.push_str(" world");
}
