use crate::{buffer::Buffer, window::WinBuffer};
use std::{fmt, result};

pub type Marks = [Option<Mark>; 256];

pub fn new_marks() -> Marks {
    let mut marks: Marks = unsafe { std::mem::zeroed() };

    (0..marks.len()).for_each(|i| marks[i] = None);
    marks
}

#[derive(Clone, Eq, PartialEq)]
pub struct Mark {
    index: char,
    buf_id: String,
    cursor: usize,
    col: usize,
    row: usize,
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Mark<{:?},{},{}>", self.index, self.buf_id, self.cursor)
    }
}

impl From<char> for Mark {
    fn from(index: char) -> Mark {
        Mark {
            index: index,
            buf_id: String::default(),
            cursor: usize::default(),
            col: usize::default(),
            row: usize::default(),
        }
    }
}

impl Mark {
    pub fn into_mark(mut self, buf: &Buffer) -> Self {
        self.buf_id = buf.to_id();
        self.cursor = buf.to_char_cursor();

        let bc_xy = buf.to_xy_cursor(Some(self.cursor));

        self.col = bc_xy.col;
        self.row = bc_xy.row;
        self
    }

    #[inline]
    pub fn to_buffer_id(&self) -> String {
        self.buf_id.clone()
    }

    #[inline]
    pub fn to_cursor(&self) -> usize {
        self.cursor
    }

    #[inline]
    pub fn to_index(&self) -> char {
        self.index
    }

    #[inline]
    pub fn to_row(&self) -> usize {
        self.row
    }
}

#[inline]
pub fn get_mark(marks: &Marks, index: char) -> Option<Mark> {
    marks[index as usize].clone()
}

#[inline]
pub fn set_mark(marks: &mut Marks, mark: Mark) -> Option<Mark> {
    let index = mark.index as usize;
    let old = marks[index].take();
    marks[index] = Some(mark);
    old
}

#[inline]
pub fn del_mark(marks: &mut Marks, index: u8) -> Option<Mark> {
    marks[index as usize].take()
}
