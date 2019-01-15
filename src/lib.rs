//!
//! The `lsdiff` library.
//!

use failure::Fail;

pub struct Entry {
    pub start_line: usize,
    pub hunk_start_line: usize,
    pub file_path: String,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "The patch is malformed at line {}", _0)]
    MalformedPatch(usize),
}

pub type LsdiffResult<T> = Result<T, Error>;

pub fn process(patch: &str) -> LsdiffResult<Vec<Entry>> {
    let mut entries = Vec::new();

    let lines: Vec<&str> = patch.split('\n').map(|line| line.trim()).collect();
    for (index, line) in lines.iter().enumerate() {
        if line.starts_with("---") {
            let elements: Vec<&str> = line.split(' ').collect();
            if elements.len() < 2 {
                return Err(Error::MalformedPatch(index + 1));
            }
            entries.push(Entry {
                start_line: index + 1,
                hunk_start_line: index + 1,
                file_path: elements[1].to_owned(),
            });
        }
    }

    Ok(entries)
}
