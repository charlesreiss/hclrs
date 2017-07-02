use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str;

use errors::Error;

// FIXME: untested code; use this to make good error messages
pub struct FileContents {
    data: String,
    filenames: Vec<(usize, String)>,
    newlines: Vec<(usize, usize)>,
}

fn mark_newlines(offset: usize, newlines_list: &mut Vec<(usize, usize)>, data: &str) {
    newlines_list.push((offset + 0, 1));
    let mut index = 1;
    for (i, _) in data.match_indices('\n') {
        index += 1;
        newlines_list.push((offset + i, index));
    }
}

impl FileContents {
    pub fn new_from_file_with_preamble(preamble: &str, path: &Path) -> Result<FileContents, Error> {
        let file = File::open(path)?;
        let mut file_reader = BufReader::new(file);
        let mut contents = String::from(preamble);
        let filename = path.file_name().map(|x| x.to_string_lossy().into_owned()).unwrap_or(String::from("<unknown>"));
        let mut newlines = Vec::new();
        mark_newlines(0, &mut newlines, &contents);
        let mut file_bytes = Vec::new();
        file_reader.read_to_end(&mut file_bytes)?;
        // FIXME: error if bytes aren't UTF-8?
        match str::from_utf8(&file_bytes) {
            Ok(_) => {},
            Err(bad_utf8) => {
                warn!("input file {} is not valid UTF-8", filename);
            },
        };
        contents.push_str(&String::from_utf8_lossy(&file_bytes));
        mark_newlines(preamble.len(), &mut newlines, contents.split_at(preamble.len()).1);
        let filenames = vec!(
            (0, String::from("<builtin>")),
            (preamble.len(), filename)
        );
        Ok(FileContents {
            data: contents,
            filenames: filenames,
            newlines: newlines,
        })
    }

    pub fn new_from_file(path: &Path) -> Result<FileContents, Error> {
        FileContents::new_from_file_with_preamble("\n", path)
    }

    pub fn data(&self) -> &str { &self.data }

    pub fn filename(&self, index: usize) -> &str {
        let index = match self.filenames.binary_search_by_key(&index, |ref x| x.0) {
            Ok(x) => x,
            Err(x) => x - 1
        };
        &self.filenames[index].1
    }

    pub fn line_number_and_bounds(&self, index: usize) -> (usize, usize, usize) {
        let index = match self.newlines.binary_search_by_key(&index, |ref x| x.0) {
            Ok(x) => x,
            Err(x) => x - 1
        };
        let next_line_loc = if index == self.newlines.len() - 1 { self.data.len() } else { self.newlines[index + 1].0 };
        let cur_line = self.newlines[index];
        (cur_line.1, cur_line.0, next_line_loc)
    }

    pub fn line(&self, index: usize) -> usize {
        self.line_number_and_bounds(index).1
    }

    pub fn file_and_line(&self, index: usize) -> String {
        let filename = self.filename(index);
        let line = self.line(index);
        format!("{}:{}", filename, line)
    }

    pub fn range(&self, start: usize, end: usize) -> String {
        let filename = self.filename(start);
        let start_line = self.line(start);
        let end_line = self.line(end);
        if start_line == end_line {
            format!("{}:{}", filename, start_line)
        } else {
            format!("{}:{}-{}", filename, start_line, end_line)
        }
    }

    pub fn show_region(&self, start: usize, end: usize) -> String {
        let filename = self.filename(start);
        let (begin_line_no, begin_loc, _) = self.line_number_and_bounds(start);
        let (_, _, end_loc) = self.line_number_and_bounds(end);
        let segment = &self.data[begin_loc..end_loc];
        let mut result = String::new();
        // FIXME: variable width line count
        result.push_str(&format!("     -> {}:{}\n", filename, begin_line_no));
        result.push_str(         "     |\n");
        let mut number = begin_line_no;
        for line in segment.lines() {
            result.push_str(&format!("{:4} | {}\n", number, line));
            number += 1;
        }
        result
    }
}

