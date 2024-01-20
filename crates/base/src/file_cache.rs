use crate::source_id::SourceId;
use codespan_reporting::files::Error as CodeSpanError;
use codespan_reporting::files::{line_starts, Files};
use std::collections::{hash_map::Entry, HashMap};
use std::fs;
use std::io::Error as IoError;
use std::ops::Range;

/// `FileCache` is a simple cache for storing and fetching file contents.
#[derive(Default, Debug, Clone)]
pub struct FileCache {
    files: HashMap<SourceId, String>,
}

impl FileCache {
    pub fn new(files: HashMap<SourceId, String>) -> Self {
        Self { files }
    }

    pub fn empty() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    pub fn sources(&self) -> impl Iterator<Item = &SourceId> {
        self.files.keys()
    }

    /// Fetches the contents of a file associated with the given `SourceId`.
    ///
    /// If the file is not present in the cache, it is read from disk and added to the cache.
    ///
    /// ### Arguments
    ///
    /// * `src` - A reference to the `SourceId` of the file to fetch.
    ///
    /// ### Returns
    ///
    /// A `Result` containing a reference to the file's content as a `&str`, or a `FileCacheError` if an error occurs.
    pub fn fetch(&mut self, src: SourceId) -> Result<&str, FileCacheError> {
        Ok(match self.files.entry(src) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let fc = fs::read_to_string(src.to_path())?;
                entry.insert(fc)
            }
        })
    }

    pub fn load(&mut self, src: SourceId) -> Result<(), FileCacheError> {
        self.fetch(src).map(|_| ())
    }
}

impl<'a> Files<'a> for FileCache {
    type FileId = SourceId;
    type Name = String;
    type Source = &'a str;

    fn name(&'a self, id: Self::FileId) -> Result<Self::Name, CodeSpanError> {
        Ok(id.to_string())
    }

    fn source(&'a self, id: Self::FileId) -> Result<Self::Source, CodeSpanError> {
        let opt = self.files.get(&id).map(|s| s.as_str());
        opt.ok_or(CodeSpanError::FileMissing)
    }

    fn line_index(&'a self, id: Self::FileId, byte_index: usize) -> Result<usize, CodeSpanError> {
        let s = self.source(id)?;

        let line_starts = line_starts(s).collect::<Vec<_>>();
        match line_starts.binary_search(&byte_index) {
            Ok(line) => Ok(line),
            Err(next_line) => Ok(next_line - 1),
        }
    }

    fn line_range(
        &'a self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>, CodeSpanError> {
        let s = self.source(id)?;

        let line_starts = line_starts(s).collect::<Vec<_>>();
        let start = line_starts.get(line_index).copied().unwrap();
        let end = line_starts.get(line_index + 1).copied().unwrap_or(s.len());
        Ok(start..end)
    }
}

#[derive(Debug)]
pub enum FileCacheError {
    IoError { io_err: IoError },
}

impl From<IoError> for FileCacheError {
    fn from(error: IoError) -> Self {
        FileCacheError::IoError { io_err: error }
    }
}
