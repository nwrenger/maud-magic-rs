use light_magic::atomic::{AtomicDatabase, DataStore};
use light_magic::serde::{Deserialize, Serialize};
use light_magic::table::{PrimaryKey, Table};
use std::path::Path;
use std::sync::Arc;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Database {
    pub books: Table<Book>,
}

impl DataStore for Database {}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: usize,
    pub title: String,
    pub author: String,
    pub price: f64,
}

impl PrimaryKey for Book {
    type PrimaryKeyType = usize;

    fn primary_key(&self) -> &Self::PrimaryKeyType {
        &self.id
    }
}

pub fn init(path: &Path) -> Arc<AtomicDatabase<Database>> {
    let db = Database::open(path);
    db.into()
}
