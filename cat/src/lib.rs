use std::fs::{self};
use std::io::Result as IOResult;

pub struct FileContent {
    file_name: String,
}

impl FileContent {
    pub fn new(file_name: impl Into<String>) -> Self {
        FileContent {
            file_name: file_name.into(),
        }
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn read(&self) -> IOResult<Vec<u8>> {
        fs::read(&self.file_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_content_name() {
        let fc = FileContent::new(".gitignore");
        assert_eq!(fc.file_name(), ".gitignore");
    }
}
