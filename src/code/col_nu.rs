use std::{cmp, fmt, iter::FromIterator, result};

use crate::{
    color_scheme::{ColorScheme, Highlight},
    window::Span,
};

#[derive(Clone)]
/// Line number rendering. Starts from 1 till last line the buffer, width is
/// padded with adequate spaces on the left, and one space to the right.
pub struct ColNu {
    width: u16,
}

impl fmt::Display for ColNu {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Nu<{}>", self.width)
    }
}

impl ColNu {
    pub fn new(mut line_idx: usize) -> Self {
        let width = {
            use crate::buffer::MAX_LINES;

            assert!(line_idx < MAX_LINES, "assert {}", line_idx);
            // line number rendering starts from 1..
            line_idx += 1;
            cmp::max(line_idx.to_string().len(), 4) as u16
        };

        ColNu { width }
    }

    pub fn to_width(&self) -> u16 {
        self.width
    }

    pub fn to_span(&self, nu: Option<usize>, scheme: &ColorScheme) -> Span {
        use std::iter::repeat;

        let s = match nu {
            Some(nu) => format!("{:>width$} ", nu, width = (self.width as usize)),
            None => String::from_iter(repeat(' ').take(self.width as usize)),
        };
        let span: Span = s.into();
        span.using(scheme.to_style(Highlight::LineNr))
    }
}
