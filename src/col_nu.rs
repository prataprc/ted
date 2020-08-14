#[allow(unused_imports)]
use log::debug;

use std::{fmt, result};

use crate::{
    colors::{ColorScheme, Highlight},
    term::{Span, Style},
};

#[derive(Clone, Copy, Eq, PartialEq)]
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

impl ColKind {
    pub fn is_empty(&self) -> bool {
        match self {
            ColKind::Nu(_) => false,
            ColKind::Wrap => false,
            ColKind::Empty => true,
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
    pub fn new(line_idx: usize, line_number: bool) -> Self {
        let width = {
            use crate::buffer::MAX_LINES;

            assert!(line_idx < MAX_LINES, "assert {}", line_idx);
            // line number rendering starts from 1..
            match (line_idx + 1).to_string().len() {
                0 | 1 | 2 => 3 + 1,
                n => n + 1,
            }
        };

        ColNu {
            width: width as u16,
            line_number,
            style_line_nr: Style::default(),
            style_empty: Style::default(),
        }
    }

    pub fn set_color_scheme(&mut self, scheme: &ColorScheme) -> &mut Self {
        let mut empty = scheme.to_style(Highlight::Canvas);
        self.style_line_nr = scheme.to_style(Highlight::LineNr);
        empty.fg = self.style_line_nr.fg.clone();
        self.style_empty = empty;
        self
    }

    #[inline]
    pub fn to_width(&self) -> u16 {
        if_else!(self.line_number, self.width, 0)
    }

    pub fn to_span(&self, nu: ColKind) -> Span {
        use ColKind::{Empty, Nu, Wrap};

        let width = (self.width as usize).saturating_sub(1);

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
