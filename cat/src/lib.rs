use std::{
    fmt,
    fs::{self},
    io::{self, prelude::*},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Input {
    File(String),
    Stdin,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FileContent {
    input: Input,
}

impl FileContent {
    pub fn from_file_name(file_name: impl Into<String>) -> Self {
        FileContent {
            input: Input::File(file_name.into()),
        }
    }

    pub fn from_stdin() -> Self {
        FileContent {
            input: Input::Stdin,
        }
    }

    pub fn file_name(&self) -> Option<&str> {
        if let Input::File(s) = &self.input {
            return Some(s);
        }

        None
    }

    pub fn read(&self) -> io::Result<Vec<u8>> {
        match &self.input {
            Input::File(file_name) => fs::read(file_name),
            Input::Stdin => {
                let stdin = std::io::stdin();
                let mut stdin = stdin.lock();
                let mut buf = vec![];
                stdin.read_to_end(&mut buf)?;
                Ok(buf)
            }
        }
    }
}

impl fmt::Debug for FileContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(FileContent))
            .field("input", &self.input)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_name() {
        let fc = FileContent::from_file_name(".gitignore");
        assert_eq!(fc.file_name(), Some(".gitignore"));

        let fc = FileContent::from_stdin();
        assert_eq!(fc.file_name(), None);
    }

    #[test]
    fn test_debug() {
        let fc = FileContent::from_file_name(".gitignore");
        assert_eq!(
            format!("{:?}", fc),
            "FileContent { input: File(\".gitignore\") }"
        );

        let fc = FileContent::from_stdin();
        assert_eq!(format!("{:?}", fc), "FileContent { input: Stdin }");
    }
}
