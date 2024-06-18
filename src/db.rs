use light_magic::db;
use std::sync::Arc;

db! {
    Table<Book> => { id: usize, title: String, author: String, price: f64 }
}

pub fn init(path: &Path) -> Arc<AtomicDatabase<Database>> {
    let db = Database::open(path);
    db.into()
}
