use std::{
    fs::File,
    io::{self, prelude::*},
};

use io::BufReader;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Input {
    File(String),
    Stdin,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct InputContent {
    input: Input,
}

impl InputContent {
    pub fn from_file_name(file_name: impl Into<String>) -> Self {
        InputContent {
            input: Input::File(file_name.into()),
        }
    }

    pub fn from_stdin() -> Self {
        InputContent {
            input: Input::Stdin,
        }
    }

    pub fn file_name(&self) -> Option<&str> {
        match &self.input {
            Input::Stdin => None,
            Input::File(s) => Some(s),
        }
    }

    pub fn read_lines(&self, line_count: usize) -> io::Result<Vec<io::Result<String>>> {
        let range = 0..line_count;

        match &self.input {
            Input::File(file_name) => {
                let f = File::open(file_name)?;
                let mut f = BufReader::new(f);
                Ok(range
                    .map(move |_| {
                        let mut buf = String::new();
                        f.read_line(&mut buf).map(|_| buf)
                    })
                    .collect())
            }
            Input::Stdin => {
                let stdin = io::stdin();
                let mut stdin = stdin.lock();
                Ok(range
                    .map(move |_| {
                        let mut buf = String::new();
                        stdin.read_line(&mut buf).map(|_| buf)
                    })
                    .collect())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_debug() {
        let fc = InputContent::from_file_name(".gitignore");
        assert_eq!(
            format!("{:?}", fc),
            "InputContent { input: File(\".gitignore\") }"
        );

        let fc = InputContent::from_stdin();
        assert_eq!(format!("{:?}", fc), "InputContent { input: Stdin }");
    }

    #[test]
    fn test_file_name() {
        let fc = InputContent::from_file_name(".gitignore");
        assert_eq!(fc.file_name(), Some(".gitignore"));

        let fc = InputContent::from_stdin();
        assert_eq!(fc.file_name(), None);
    }
}
