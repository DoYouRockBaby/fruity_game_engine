use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileExplorerState {
    current_dir: String,
}

impl Default for FileExplorerState {
    fn default() -> Self {
        FileExplorerState {
            current_dir: "./assets".to_string(),
        }
    }
}

impl FileExplorerState {
    pub fn get_current_dir(&self) -> String {
        self.current_dir.clone()
    }

    pub fn open_dir(&mut self, path: &str) {
        self.current_dir = path.to_string();
    }

    pub fn get_files(&self) -> Vec<PathBuf> {
        match fs::read_dir(&self.current_dir) {
            Ok(dir) => dir
                .filter_map(|file| file.ok())
                .map(|file| file.path())
                .collect::<Vec<_>>(),
            Err(_) => Vec::new(),
        }
    }
}
