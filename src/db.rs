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
    
    let book3 = Book {
        id: 3,
        title: "Rust by Example".to_string(),
        author: "Steve Klabnik".to_string(),
        price: 29.99,
    };
    
    let book4 = Book {
        id: 4,
        title: "Rust for Rustaceans".to_string(),
        author: "Jon Gjengset".to_string(),
        price: 49.99,
    };
    
    let book5 = Book {
        id: 5,
        title: "Clean Code".to_string(),
        author: "Robert C. Martin".to_string(),
        price: 32.99,
    };

    let book6 = Book {
        id: 6,
        title: "The Pragmatic Programmer".to_string(),
        author: "Andrew Hunt and David Thomas".to_string(),
        price: 37.99,
    };

    let book7 = Book {
        id: 7,
        title: "You Don't Know JS".to_string(),
        author: "Kyle Simpson".to_string(),
        price: 24.99,
    };

    let book8 = Book {
        id: 8,
        title: "Design Patterns: Elements of Reusable Object-Oriented Software".to_string(),
        author: "Erich Gamma, Richard Helm, Ralph Johnson, and John Vlissides".to_string(),
        price: 54.99,
    };

    db.insert_book(book1);
    db.insert_book(book2);
    db.insert_book(book3);
    db.insert_book(book4);
    db.insert_book(book5);
    db.insert_book(book6);
    db.insert_book(book7);
    db.insert_book(book8);

    db
}
