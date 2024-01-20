use crate::source_id::SourceId;
use std::{
    fmt::{self, Display},
    ops::Range,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Spanned<T> {
    pub span: Range<usize>,
    pub value: T,
}

impl<T> Spanned<T> {
    pub fn new(span: Range<usize>, value: T) -> Self {
        Self { span, value }
    }

    pub fn with_new_value<U>(&self, value: U) -> Spanned<U> {
        Spanned {
            span: self.span.clone(),
            value,
        }
    }
}

impl<A> Spanned<A> {
    pub fn map_value<U, F>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(A) -> U,
    {
        Spanned {
            span: self.span,
            value: f(self.value),
        }
    }
}

impl Spanned<()> {
    pub fn empty(span: Range<usize>) -> Self {
        Self { span, value: () }
    }
}

impl<T> Display for Spanned<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

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
