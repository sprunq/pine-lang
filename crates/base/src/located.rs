use crate::{source_id::SourceId, spanned::Spanned};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Located<T> {
    pub source: SourceId,
    pub located: Spanned<T>,
}

impl<T> Located<T> {
    pub fn new(source: SourceId, value: Spanned<T>) -> Self {
        Self {
            source,
            located: value,
        }
    }
}

impl<T> Copy for Located<T> where T: Copy {}
