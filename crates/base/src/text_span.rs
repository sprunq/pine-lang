use std::ops::Range;

/// A pseudo `Range<usize>` that implements Copy and uses `u32`.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct TextSpan {
    pub start: u32,
    pub end: u32,
}

impl TextSpan {
    pub fn new<I>(start: I, end: I) -> Self
    where
        I: Into<u32>,
    {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.end - self.start
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl From<Range<usize>> for TextSpan {
    fn from(range: Range<usize>) -> Self {
        Self::new(range.start as u32, range.end as u32)
    }
}

impl From<Range<u32>> for TextSpan {
    fn from(range: Range<u32>) -> Self {
        Self::new(range.start, range.end)
    }
}

impl From<Range<i32>> for TextSpan {
    fn from(range: Range<i32>) -> Self {
        Self::new(range.start as u32, range.end as u32)
    }
}

impl From<TextSpan> for Range<usize> {
    fn from(span: TextSpan) -> Self {
        span.start as usize..span.end as usize
    }
}
