use std::{fmt::Display, ops::Range};

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

impl<S> Located<S>
where
    S: Into<String> + Clone,
{
    pub fn as_str_loc(&self) -> Located<String> {
        let span = self.span.clone();
        let source = self.source;
        let value = self.value.clone().into();
        Located::new(source, span, value)
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

impl Into<Range<usize>> for Located<()> {
    fn into(self) -> Range<usize> {
        self.span
    }
}

impl<T> Display for Located<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
