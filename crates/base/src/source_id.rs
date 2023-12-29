use internment::Intern;
use std::{
    fmt,
    path::{Path, PathBuf},
};

/// `SourceId` is a wrapper around an interning data structure that represents
/// a file's path in a more memory-efficient way.
/// Can be copied without much overhead.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SourceId(Intern<Vec<String>>);

impl SourceId {
    /// A new `SourceId` instance representing the given file path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        SourceId(Intern::new(
            path.as_ref()
                .iter()
                .map(|c| c.to_string_lossy().into_owned())
                .collect(),
        ))
    }

    /// Converts the `SourceId` back to a `PathBuf`.
    pub fn to_path(&self) -> PathBuf {
        self.0.iter().map(|e| e.to_string()).collect()
    }

    /// Returns the cloned components of the path.
    pub fn components(&self) -> Vec<String> {
        (*self.0).clone()
    }

    /// Returns the filename of the path without the extension.
    pub fn filename(&self) -> String {
        let mut components = self.components();
        let filename = components.pop().unwrap();
        let filename = filename.split('.').collect::<Vec<_>>();
        filename[filename.len() - 2].to_string()
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.len() == 0 {
            write!(f, "?")
        } else {
            write!(f, "{}", self.0.clone().join("/"))
        }
    }
}

impl fmt::Debug for SourceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
