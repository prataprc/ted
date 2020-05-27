use std::{cmp, fmt, result};

use crate::{
    color_scheme::{ColorScheme, Highlight},
    window::Span,
};

#[derive(Clone)]
/// Line number rendering. Starts from 1 till last line the buffer, width is
/// padded with adequate spaces on the left, and one space to the right.
pub struct ColNu {
    width: u16,
    line_number: bool,
}

impl fmt::Display for ColNu {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Nu<{}>", self.width)
    }
}

impl ColNu {
    pub fn new(mut line_idx: usize, line_number: bool) -> Self {
        let width = {
            use crate::buffer::MAX_LINES;

            assert!(line_idx < MAX_LINES, "assert {}", line_idx);
            // line number rendering starts from 1..
            line_idx += 1;
            cmp::max(line_idx.to_string().len(), 4) as u16
        };

        ColNu { width, line_number }
    }

    pub fn to_width(&self) -> u16 {
        if self.line_number {
            self.width
        } else {
            0
        }
    }

    pub fn to_span(&self, nu: Option<usize>, scheme: &ColorScheme) -> Span {
        let s = match nu {
            Some(nu) if self.line_number => {
                let width = self.width as usize;
                format!("{:>width$} ", nu, width = width)
            }
            Some(_) => "".to_string(),
            None => "~".to_string(),
        };
        let span: Span = s.into();
        span.using(scheme.to_style(Highlight::LineNr))
    }
}
