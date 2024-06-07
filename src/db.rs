use light_magic::db;
use serde::Deserialize;

db! {
    book: [Deserialize] => { id: usize, title: String, author: String, price: f64 }
}

pub fn init() -> Database {
    let db = Database::new();

    let book1 = Book {
        id: 1,
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        price: 39.99,
    };

    let book2 = Book {
        id: 2,
        title: "Programming Rust".to_string(),
        author: "Jim Blandy and Jason Orendorff".to_string(),
        price: 44.99,
    };

    db.insert_book(book1);
    db.insert_book(book2);

    db
}
