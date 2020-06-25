use std::{cmp, fmt, result};

use crate::{
    colors::{ColorScheme, Highlight},
    term::{Span, Style},
};

#[derive(Clone)]
pub enum ColKind {
    Nu(usize),
    Wrap,
    Empty,
}

impl fmt::Display for ColKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            ColKind::Nu(nu) => write!(f, "ColKind::Nu<{}>", nu),
            ColKind::Wrap => write!(f, "ColKind::Wrap"),
            ColKind::Empty => write!(f, "ColKind::Empty"),
        }
    }
}

impl fmt::Debug for ColKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            ColKind::Nu(nu) => write!(f, "ColKind::Nu<{}>", nu),
            ColKind::Wrap => write!(f, "ColKind::Wrap"),
            ColKind::Empty => write!(f, "ColKind::Empty"),
        }
    }
}

#[derive(Clone)]
/// Line number rendering. Starts from 1 till last line the buffer, width is
/// padded with adequate spaces on the left, and one space to the right.
pub struct ColNu {
    width: u16,
    line_number: bool,
    style_line_nr: Style,
    style_empty: Style,
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

        ColNu {
            width,
            line_number,
            style_line_nr: Default::default(),
            style_empty: Default::default(),
        }
    }

    pub fn set_color_scheme(&mut self, scheme: &ColorScheme) -> &mut Self {
        let mut empty = scheme.to_style(Highlight::Canvas);
        self.style_line_nr = scheme.to_style(Highlight::LineNr);
        empty.fg = self.style_line_nr.fg.clone();
        self.style_empty = empty;
        self
    }

    pub fn to_width(&self) -> u16 {
        if self.line_number {
            self.width
        } else {
            0
        }
    }

    pub fn to_span(&self, nu: ColKind) -> Span {
        use ColKind::{Empty, Nu, Wrap};

        let width = (self.width as usize) - 1;

        match nu {
            Nu(nu) if self.line_number => {
                let span: Span = format!("{:>w$} ", nu, w = width).into();
                span.using(self.style_line_nr.clone())
            }
            Wrap if self.line_number => {
                let span: Span = format!("{:>w$} ", "", w = width).into();
                span.using(self.style_line_nr.clone())
            }
            Empty => {
                let span: Span = format!("{:<w$} ", "~", w = width).into();
                span.using(self.style_empty.clone())
            }
            _ => "".to_string().into(),
        }
    }
}
