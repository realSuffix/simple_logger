use chrono::prelude::*;
use std::collections::HashMap;
use std::io::Error as IOError;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct Logger<T>
where
    T: std::cmp::Eq,
{
    mappings: HashMap<T, PathBuf>,
}

impl<T> Logger<T>
where
    T: std::cmp::Eq + std::hash::Hash,
{
    pub fn add_mapping(&mut self, identifier: T, directory: PathBuf) {
        self.mappings.insert(identifier, directory);
    }

    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    pub fn log_to(&mut self, identifier: T, msg: &str) -> Result<(), IOError> {
        if let Some(path) = self.mappings.get_mut(&identifier) {
            let file_name: String = Local::now().format("%F").to_string() + ".log";
            path.push(file_name);
            let mut file = OpenOptions::new().append(true).create(true).open(&path)?;
            writeln!(file, "{}", msg)?;
            path.pop();
        }
        Ok(())
    }
}
