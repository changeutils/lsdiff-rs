//!
//! The `lsdiff` library.
//!

use failure::Fail;

#[derive(Default)]
pub struct Entry {
    /// Starting from 0
    pub start_line: usize,
    /// Starting from 0
    pub hunk_start_line: usize,
    /// The file being patched path
    pub input_path: String,
    /// The patched file path
    pub output_path: String,
    /// Between consequent `start_line`s
    pub lines_count: usize,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "The patch is malformed at line {}", _0)]
    MalformedPatch(usize),
}

pub type LsdiffResult<T> = Result<T, Error>;

pub fn process(patch: &str) -> LsdiffResult<Vec<Entry>> {
    let lines: Vec<&str> = patch.split('\n').map(|line| line.trim()).collect();

    let mut entries: Vec<Entry> = Vec::new();

    for index in 0..lines.len() - 1 {
        if !(lines[index].starts_with("---") && lines[index + 1].starts_with("+++")) {
            continue;
        }

        let mut entry = Entry::default();
        entry.start_line = index;
        entry.hunk_start_line = index + 2;

        if let Some(last) = entries.last_mut() {
            last.lines_count = entry.start_line - last.start_line;
        }

        let elements: Vec<&str> = lines[index].split(' ').collect();
        if elements.len() < 2 {
            return Err(Error::MalformedPatch(index + 1));
        }
        entry.input_path = elements[1].to_owned();

        let elements: Vec<&str> = lines[index + 1].split(' ').collect();
        if elements.len() < 2 {
            return Err(Error::MalformedPatch(index + 2));
        }
        entry.output_path = elements[1].to_owned();

        entries.push(entry)
    }

    if let Some(last) = entries.last_mut() {
        last.lines_count = lines.len() - 1 - last.start_line;
    }

    Ok(entries)
}
