use crossterm::style::{Attribute, Color};

use std::{cmp, fmt, iter::FromIterator, result};

use crate::window::Span;

#[derive(Clone, Copy)]
pub struct ColNu {
    width: u16,
    fg: Color,
    bg: Color,
    attr: Attribute,
}

impl fmt::Display for ColNu {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Nu<{}>", self.width)
    }
}

impl ColNu {
    pub fn new(line_idx: usize) -> Self {
        ColNu {
            width: compute_nu_width(line_idx + 1),
            fg: Color::Rgb {
                r: 135,
                g: 135,
                b: 95,
            },
            bg: Color::Rgb {
                r: 153,
                g: 152,
                b: 114,
            },
            attr: Attribute::NormalIntensity,
        }
    }

    pub fn to_width(&self) -> u16 {
        self.width
    }

    pub fn to_span(&self, nu: Option<usize>) -> Span {
        use std::iter::repeat;

        let s = match nu {
            Some(nu) => format!("{:>width$} ", nu, width = (self.width as usize)),
            None => String::from_iter(repeat(' ').take(self.width as usize)),
        };
        let span: Span = s.into();
        span.with(self.fg).on(self.bg).attribute(self.attr)
    }
}

fn compute_nu_width(line_idx: usize) -> u16 {
    use crate::buffer::MAX_LINES;

    assert!(line_idx < MAX_LINES);
    (cmp::max(line_idx.to_string().len(), 2) + 1) as u16
}
