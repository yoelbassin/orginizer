use once_cell::sync::OnceCell;
use serde_json::{self, Map, Value};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader};
use std::path::PathBuf;
use std::sync::RwLock;

pub struct Cache {
    path: PathBuf,
    data: RwLock<Map<String, Value>>,
}

impl Cache {
    pub fn new(path: PathBuf) -> Self {
        let data = Self::load_cache_entries(&path).unwrap_or_else(|_| Map::new());
        Self {
            path,
            data: RwLock::new(data),
        }
    }

    pub fn get_or_insert_with<F>(&self, key: &str, f: F) -> io::Result<String>
    where
        F: FnOnce() -> String,
    {
        // First, try to read with a shared lock
        {
            let data = self.data.read().unwrap();
            if let Some(value) = data.get(key) {
                if let Some(str_value) = value.as_str() {
                    return Ok(str_value.to_string());
                }
            }
        }

        // If not found, upgrade to a write lock
        let mut data = self.data.write().unwrap();
        // Double-check in case another thread inserted it
        if let Some(value) = data.get(key) {
            if let Some(str_value) = value.as_str() {
                return Ok(str_value.to_string());
            }
        }
        // Key doesn't exist or value is not a string, compute new value
        let new_value = f();
        data.insert(key.to_string(), Value::String(new_value.clone()));
        self.write_cache_entries(&data)?;
        Ok(new_value)
    }

    fn load_cache_entries(path: &PathBuf) -> io::Result<Map<String, Value>> {
        if !path.exists() {
            return Ok(Map::new());
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let value: Value = serde_json::from_reader(reader)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        match value {
            Value::Object(map) => Ok(map),
            _ => Ok(Map::new()),
        }
    }

    fn write_cache_entries(&self, cache_data: &Map<String, Value>) -> io::Result<()> {
        // Create parent directories if they don't exist
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;
        serde_json::to_writer_pretty(file, &Value::Object(cache_data.clone()))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(())
    }
}

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
