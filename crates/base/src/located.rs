use crate::source_id::SourceId;
use std::{
    fmt::{self, Display},
    ops::Range,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Located<T> {
    pub span: Range<usize>,
    pub source: SourceId,
    pub value: T,
}

impl<T> Located<T> {
    pub fn new<S>(source: S, span: Range<usize>, value: T) -> Self
    where
        S: Into<SourceId>,
    {
        Self {
            span,
            source: source.into(),
            value,
        }
    }

    pub fn with_new_value<U>(&self, value: U) -> Located<U> {
        Located {
            span: self.span.clone(),
            source: self.source,
            value,
        }
    }
}

impl<A> Located<A> {
    pub fn map_value<U, F>(&self, f: F) -> Located<U>
    where
        F: FnOnce(&A) -> U,
    {
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
