use std::{
    fmt::{self, Display},
    ops::Range,
};

use crate::source_id::SourceId;

#[derive(Debug, Clone, PartialEq)]
pub struct Located<T> {
    pub span: Range<usize>,
    pub source: SourceId,
    pub value: T,
}

impl<T> Located<T> {
    pub fn new(source: SourceId, span: Range<usize>, value: T) -> Self {
        Self {
            span,
            source,
            value,
        }
    }
}

impl<A> Located<A> {
    pub fn map_value<U, F: FnOnce(&A) -> U>(&self, f: F) -> Located<U> {
        Located {
            span: self.span.clone(),
            source: self.source,
            value: f(&self.value),
        }
    }
}

impl Located<()> {
    pub fn empty(source: SourceId, span: Range<usize>) -> Self {
        Self {
            span,
            source,
            value: (),
        }
    }
}

impl<T> Display for Located<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
