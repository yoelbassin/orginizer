use sled::Db;
use std::io;
use std::path::PathBuf;

pub struct Cache {
    db: Db,
}

impl Cache {
    pub fn new(path: PathBuf) -> Self {
        let db = sled::open(path).expect("Failed to open sled database");
        Self { db }
    }

    pub fn get_or_insert_with<F>(&self, key: &str, f: F) -> io::Result<String>
    where
        F: FnOnce() -> String,
    {
        if let Ok(Some(value)) = self.db.get(key) {
            // Value found, return as String
            Ok(String::from_utf8(value.to_vec()).unwrap())
        } else {
            // Not found, compute and insert
            let new_value = f();
            self.db.insert(key, new_value.as_bytes()).unwrap();
            self.db.flush().unwrap(); // Optional: flush to disk immediately
            Ok(new_value)
        }
    }
}

use once_cell::sync::OnceCell;

static CACHE: OnceCell<Cache> = OnceCell::new();

pub fn init_cache(path: PathBuf) {
    let _ = CACHE.set(Cache::new(path));
}

pub fn cache_get_or_insert_with<F>(key: &str, f: F) -> io::Result<String>
where
    F: FnOnce() -> String,
{
    CACHE.get().unwrap().get_or_insert_with(key, f)
}
