use once_cell::sync::OnceCell;
use serde_json::{self, Map, Value};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader};
use std::path::PathBuf;

pub struct Cache {
    path: PathBuf,
}

impl Cache {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn get_or_insert_with<F>(&self, key: &str, f: F) -> io::Result<String>
    where
        F: FnOnce() -> String,
    {
        let mut cache_data = self.read_cache_entries()?;

        if let Some(value) = cache_data.get(key) {
            if let Some(str_value) = value.as_str() {
                return Ok(str_value.to_string());
            }
        }

        // Key doesn't exist or value is not a string, compute new value
        let new_value = f();
        cache_data.insert(key.to_string(), Value::String(new_value.clone()));
        self.write_cache_entries(&cache_data)?;

        Ok(new_value)
    }

    fn read_cache_entries(&self) -> io::Result<Map<String, Value>> {
        if !self.path.exists() {
            return Ok(Map::new());
        }

        let file = File::open(&self.path)?;
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
