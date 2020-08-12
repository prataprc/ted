#[allow(unused_imports)]
use log::trace;

use std::{fmt, iter::FromIterator, result};

use crate::{
    colors::{ColorScheme, Highlight},
    event::Event,
    term::{Span, Spanline},
    window::{Coord, Cursor},
    Error, Result,
};

pub enum WindowSuggest {
    Active {
        coord: Coord,
        scheme: ColorScheme,
        suggestions: Vec<String>,
        choice: Option<usize>,
    },
    Empty {
        coord: Coord,
        scheme: ColorScheme,
    },
}

impl fmt::Display for WindowSuggest {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            WindowSuggest::Active {
                coord, suggestions, ..
            } => {
                let n = suggestions.len();
                write!(f, "WindowSuggest<active,{},{}>", coord, n)
            }
            WindowSuggest::Empty { coord, .. } => write!(f, "WindowSuggest<empty,{}>", coord),
        }
    }
}

impl WindowSuggest {
    pub fn new(coord: Coord, scheme: ColorScheme) -> Self {
        WindowSuggest::Empty { coord, scheme }
    }

    pub fn into_suggestions(self, suggestions: Vec<String>) -> Self {
        match self {
            WindowSuggest::Empty { coord, scheme } => WindowSuggest::Active {
                coord,
                scheme,
                suggestions,
                choice: None,
            },
            val @ WindowSuggest::Active { .. } => val,
        }
    }

    fn to_empty_line(coord: Coord, scheme: &ColorScheme) -> Spanline {
        use std::iter::repeat;

        let span = {
            let s = String::from_iter(repeat(' ').take(coord.wth as usize));
            let span: Span = s.into();
            span.using(scheme.to_style(Highlight::TabcLine))
        };
        span.into()
    }

    fn to_span_line<'a>(
        coord: Coord,
        scheme: &ColorScheme,
        choice: usize,
        suggestions: impl Iterator<Item = (usize, &'a String)>,
    ) -> Spanline {
        let mut spans: Vec<Span> = Vec::default();
        let iter = TabIter {
            iter: suggestions,
            wth: coord.wth,
        };
        for (i, sugg) in iter {
            let span1: Span = if choice == i {
                let span: Span = sugg.into();
                span.using(scheme.to_style(Highlight::TabcSelect))
            } else {
                let span: Span = sugg.into();
                span.using(scheme.to_style(Highlight::TabcLine))
            };
            let span2: Span = {
                let span: Span = " ".to_string().into();
                span.using(scheme.to_style(Highlight::TabcLine))
            };
            spans.push(span1);
            spans.push(span2);
        }
        Spanline::from_iter(spans.into_iter())
    }
}

impl WindowSuggest {
    #[inline]
    pub fn to_name(&self) -> String {
        "window-suggestion".to_string()
    }

    #[inline]
    pub fn to_coord(&self) -> Coord {
        match self {
            WindowSuggest::Active { coord, .. } => coord.clone(),
            WindowSuggest::Empty { coord, .. } => coord.clone(),
        }
    }

    #[inline]
    pub fn to_cursor(&self) -> Option<Cursor> {
        None
    }

    pub fn on_event(&mut self, mut evnt: Event) -> Result<Event> {
        let empty = evnt.to_modifiers().is_empty();
        evnt = match evnt {
            evnt @ Event::Tab(_) if empty => match self {
                WindowSuggest::Active {
                    choice,
                    suggestions,
                    ..
                } => {
                    *choice = match suggestions.len() {
                        0 => None,
                        n => match choice.take() {
                            Some(o) if (o + 1) == n => None,
                            Some(o) => Some(o + 1),
                            None => Some(0),
                        },
                    };
                    match choice {
                        Some(o) => Event::TabInsert(suggestions[*o].clone()),
                        None => Event::TabClear,
                    }
                }
                WindowSuggest::Empty { .. } => evnt,
            },
            evnt => evnt,
        };
        Ok(evnt)
    }

    pub fn on_refresh(&mut self) -> Result<()> {
        use crate::text;

        match self {
            WindowSuggest::Empty { .. } => (),
            WindowSuggest::Active {
                coord,
                scheme,
                choice,
                suggestions,
            } => {
                let args = match choice.as_ref() {
                    Some(0) => Some((0, 0)),
                    Some(choice) => {
                        let start = (*choice).saturating_sub(1);
                        let w: usize = {
                            let iter = suggestions.iter().skip(start);
                            iter.map(|s| text::width(s.chars())).sum()
                        };
                        let wth = coord.wth as usize;
                        Some((*choice, if_else!(w < wth, 0, start)))
                    }
                    None => None,
                };
                match args {
                    Some((choice, start)) => {
                        let mut line1 = {
                            let coord = coord.clone();
                            Self::to_empty_line(coord, &scheme)
                        };
                        line1.set_cursor(coord.to_origin_cursor().into());

                        let mut line2 = {
                            let iter = suggestions.iter().skip(start).enumerate();
                            let coord = coord.clone();
                            Self::to_span_line(coord, &scheme, choice, iter)
                        };
                        line2.set_cursor(coord.to_origin_cursor().into());

                        err_at!(Fatal, termqu!(line1, line2))?;
                    }
                    None => (),
                }
            }
        }

        Ok(())
    }
}

struct TabIter<'a, I>
where
    I: Iterator<Item = (usize, &'a String)>,
{
    iter: I,
    wth: u16,
}

impl<'a, I> Iterator for TabIter<'a, I>
where
    I: Iterator<Item = (usize, &'a String)>,
{
    type Item = (usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        use crate::text;

        match self.wth {
            0 => None,
            wth => {
                let wth = wth as usize;
                let (i, s) = self.iter.next()?;
                let s = {
                    let chars = text::take_width(s.chars(), wth);
                    String::from_iter(chars)
                };
                self.wth = wth.saturating_sub(text::width(s.chars())) as u16;
                Some((i, s))
            }
        }
    }
}
