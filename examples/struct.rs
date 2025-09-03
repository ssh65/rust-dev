#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Book {
    title: String,
    pages: u32,
}

fn main() {
    let book_1 = Book {
        title: String::from("The Rust Programming Language"),
        pages: 552,
    };

    let book_2 = book_1.clone();

    println!("Book1:{:?}", book_1);
    println!("Book2:{:?}", book_2);
}
