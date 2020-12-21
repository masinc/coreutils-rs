use std::{
    fs, io,
    iter::once,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Hash)]
pub struct LsEntry {
    path: PathBuf,
}

impl LsEntry {
    pub fn new(path: impl AsRef<Path>) -> Self {
        LsEntry {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn entries(&self) -> io::Result<Box<dyn Iterator<Item = io::Result<LsEntry>>>> {
        let meta = fs::metadata(&self.path)?;
        match meta.file_type() {
            f if f.is_file() => {
                // ファイルは自分自身をコピーして返す。
                Ok(Box::new(once(Ok(self.clone()))))
            }
            f if f.is_dir() => {
                let dir = fs::read_dir(&self.path)?;
                let entries = dir.map(|r| r.map(|entry| LsEntry::new(entry.path())));
                Ok(Box::new(entries))
            }
            f if f.is_symlink() => {
                // シンボリックリンクをトラバースしない
                Ok(Box::new(once(Ok(self.clone()))))
            }
            f => Ok(Box::new(once(Err(io::Error::new(
                io::ErrorKind::Other,
                format!("unknown error: {:?}", f),
            ))))),
        }
    }
}

impl ToString for LsEntry {
    fn to_string(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ls_entry_path() {
        let entry = LsEntry::new(".");
        assert_eq!(entry.path, PathBuf::from("."));
    }

    #[test]
    fn test_ls_entry_to_string() {
        let entry = LsEntry::new("Cargo.toml");
        assert_eq!(entry.to_string(), "Cargo.toml");
    }
}
