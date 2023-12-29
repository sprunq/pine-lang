use std::ops::Range;

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

impl Located<()> {
    pub fn empty(source: SourceId, span: Range<usize>) -> Self {
        Self {
            span,
            source,
            value: (),
        }
    }
}
