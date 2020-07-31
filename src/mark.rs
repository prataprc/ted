use crate::{buffer::Buffer, window::WinBuffer};

pub type Marks = [Option<Mark>; 256];

pub fn new_marks() -> Marks {
    let mut marks: Marks = unsafe { std::mem::zeroed() };

    (0..marks.len()).for_each(|i| marks[i] = None);
    marks
}

#[derive(Clone)]
pub struct Mark {
    index: u8,
    buf_id: String,
    cursor: usize,
    col: usize,
    row: usize,
}

impl Mark {
    pub fn new(index: u8, buf: &Buffer, cursor: usize) -> Self {
        let bc_xy = buf.to_xy_cursor(Some(cursor));
        Mark {
            index,
            buf_id: buf.to_id(),
            cursor,
            col: bc_xy.col,
            row: bc_xy.row,
        }
    }

    #[inline]
    pub fn to_cursor(&self) -> usize {
        self.cursor
    }

    #[inline]
    pub fn to_row(&self) -> usize {
        self.row
    }
}

#[inline]
pub fn get_mark(marks: &Marks, index: u8) -> Option<Mark> {
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
