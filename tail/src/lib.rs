use std::{
    fs::File,
    io::{self, prelude::*, BufReader, Cursor, Seek, SeekFrom},
};

const BUFFER_SIZE: u64 = 8 * 1024;
const DEFAULT_TAIL_LINES: usize = 10;
pub struct InputContent<R: Read + Seek> {
    reader: R,
}

impl InputContent<BufReader<File>> {
    pub fn from_file(path: impl Into<String>) -> io::Result<Self> {
        let f = File::open(path.into())?;
        let f = BufReader::new(f);
        Ok(Self::new(f))
    }
}

impl InputContent<Cursor<Vec<u8>>> {
    pub fn from_stdin() -> io::Result<Self> {
        let stdin = io::stdin();
        let mut lock = stdin.lock();
        let mut buf = vec![];
        lock.read_to_end(&mut buf)?;
        let cur = Cursor::new(buf);

        Ok(Self::new(cur))
    }
}

impl<R: Read + Seek> InputContent<R> {
    pub fn new(reader: R) -> Self {
        InputContent { reader: reader }
    }

    pub fn read_lines(&mut self, tail_lines: usize) -> io::Result<Vec<String>> {
        match self.reader.seek(SeekFrom::End(-(BUFFER_SIZE as i64))) {
            Ok(x) => x,
            Err(_) => self.reader.seek(SeekFrom::Start(0))?,
        };

        let mut s = String::new();
        self.reader.read_to_string(&mut s)?;
        let lines: Vec<_> = s.lines().collect();
        match lines.len() {
            x if x < tail_lines => Ok(lines.into_iter().map(String::from).collect()),
            _ => {
                let (_, tails) = lines.split_at(lines.len() - tail_lines);
                Ok(tails.into_iter().map(|s| String::from(*s)).collect())
            }
        }
    }

    pub fn read_lines_default(&mut self) -> io::Result<Vec<String>> {
        self.read_lines(DEFAULT_TAIL_LINES)
    }
}
