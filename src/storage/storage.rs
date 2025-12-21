use std::fs;
use std::path::PathBuf;

use crate::models::task::Task;

pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        let data_dir = dirs::data_dir()
            .expect("Could not find data directory")
            .join("lazytasks");

        if !data_dir.exists() {
            fs::create_dir_all(&data_dir).expect("Could not create data directory");
        }

        let path = data_dir.join("tasks.json");

        Self { path }
    }

    pub fn load(&self) -> Vec<Task> {
        if !self.path.exists() {
            return Vec::new();
        };

        let content = fs::read_to_string(&self.path).expect("Could not read tasks file");
        serde_json::from_str(&content).expect("Could not parse tasks file")
    }

    pub fn save(&self, tasks: &[Task]) {
        let content = serde_json::to_string_pretty(tasks).expect("Could not serialize tasks");

        let tmp_path = self.path.with_extension("json.tmp");
        fs::write(&tmp_path, content).expect("Could not write temp file");

        fs::rename(&tmp_path, &self.path).expect("Could not rename temp file");
    }
}
