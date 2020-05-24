use crossterm::style::{Attribute, Color};

use std::{cmp, fmt, iter::FromIterator, result};

use crate::window::Span;

#[derive(Clone, Copy)]
/// Line number rendering. Starts from 1 till last line the buffer, width is
/// padded with adequate spaces on the left, and one space to the right.
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
    pub fn new(mut line_idx: usize) -> Self {
        let width = {
            use crate::buffer::MAX_LINES;

            assert!(line_idx < MAX_LINES);
            // line number rendering starts from 1..
            line_idx += 1;
            cmp::max(line_idx.to_string().len(), 3) as u16
        };

        ColNu {
            width,
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
