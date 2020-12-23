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

impl fmt::Debug for InputContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(InputContent))
            .field("input", &self.input)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_name() {
        let fc = InputContent::from_file_name(".gitignore");
        assert_eq!(fc.file_name(), Some(".gitignore"));

        let fc = InputContent::from_stdin();
        assert_eq!(fc.file_name(), None);
    }

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
}
