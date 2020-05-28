use std::{cmp, fmt, result};

use crate::{
    color_scheme::{ColorScheme, Highlight},
    window::Span,
};

pub enum ColKind {
    Nu(usize),
    Wrap,
    Empty,
}

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

    pub fn to_span(&self, nu: ColKind, scheme: &ColorScheme) -> Span {
        use ColKind::{Empty, Nu, Wrap};

        match nu {
            Nu(nu) if self.line_number => {
                let width = (self.width as usize) - 1;
                let span: Span = format!("{:>w$} ", nu, w = width).into();
                span.using(scheme.to_style(Highlight::LineNr))
            }
            Wrap if self.line_number => {
                let width = (self.width as usize) - 1;
                let span: Span = format!("{:>w$} ", "", w = width).into();
                span.using(scheme.to_style(Highlight::LineNr))
            }
            Empty => {
                let span: Span = "~".to_string().into();
                span.using(scheme.to_style(Highlight::Canvas))
            }
            _ => "".to_string().into(),
        }
    }
}
