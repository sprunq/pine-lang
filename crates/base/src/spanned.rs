use crate::text_span::TextSpan;
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Spanned<T> {
    pub span: TextSpan,
    pub value: T,
}

impl<T> Spanned<T> {
    pub fn new<S>(span: S, value: T) -> Self
    where
        S: Into<TextSpan>,
    {
        Self {
            span: span.into(),
            value,
        }
    }

    pub fn map_value<U, F>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        Spanned {
            span: self.span,
            value: f(self.value),
        }
    }

    pub fn from_new_value<U>(&self, value: U) -> Spanned<U> {
        Spanned {
            span: self.span,
            value,
        }
    }
}

impl Spanned<()> {
    pub fn empty<S>(span: S) -> Self
    where
        S: Into<TextSpan>,
    {
        Self {
            span: span.into(),
            value: (),
        }
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

impl<T> Copy for Spanned<T> where T: Copy {}
