enum VShift {
    Left(usize),
    Right(usize),
    None,
}

struct WrapView {
    lines: Vec<Line>,
}

impl WrapView {
    fn new(line_idx: usize, coord: Coord, buf: &Buffer) -> WrapView {
        let mut lines = vec![];
        for (row, line_idx) in (line_idx..).take(coord.hgt).enumerate() {
            lines.push(Line::new_line(line_idx, row, coord.wth, buf))
        }
        WrapView { lines }
    }

    fn align(&self, bc: usize, cursor: Cursor) -> VShift {
        for line in self.lines.iter() {
            match line.align(bc, cursor) {
                shift @ VShift::Left(_) => return shift,
                shift @ VShift::Right(_) => return shift,
                VShift::None => (),
            }
        }
        VShift::None
    }
}

struct Line {
    nu: usize
    rows: Vec<Row>,
}

impl Line {
    fn new_line(line_idx: usize, row: u16, wth: u16, buf: &Buffer) -> Vec<Row> {
        let len_chars = buf.line(line_idx).len_chars();
        let bc = buf.line_home(line_idx);

        let rows: Vec<(u16, usize, u16, u16)> = {
            let iter = iter::repeat(wth).take(len_chars / (wth as usize));
            iter.enumerate().map(|(r, wth)| {
                assert!(r < 1_000); // TODO avoid magic number
                (row + (r as u16), bc + (r * (wth as usize)), wth, wth)
            })
        };

        if (len_chars % (wth as usize)) > 0 {
            let r = rows.len();
            let w = len_chars % (wth as usize);
            assert!(w <= (wth as usize));
            assert!(r < 1_000); // TODO avoid magic number
            rows.push((
                row + (r as u16),
                bc + (r * (wth as usize)),
                w as u16,
                wth
            ))
        }

        let rows: Vec<Row> = {
            let i1 = rows.into_iter();
            let i2 = i1.map(|(row, bc, ln, wth)| Row::new_row(row, bc, ln, wth))
            i2.collect()
        };
        Line { nu: line_idx + 1, rows }
    }

    fn align(&self, bc: usize, cursor: Cursor) -> VShift {
        for row in self.rows.iter() {
            match row.align(bc, cursor) {
                shift @ VShift::Left(_) => return shift,
                shift @ VShift::Right(_) => return shift,
                VShift::None => (),
            }
        }
        None
    }
}

struct Row {
    cells: Vec<Cell>,
}

struct Cell {
    bc: Option<usize>,
    col: u16,
    row: u16,
}

impl Row {
    fn new_row(row: u16, bc: usize, ln: u16, wth: u16) -> Row {
        let bcs: Vec<Option<usize>> = {
            let bc_end = bc+(ln as usize);
            let iter = (bc..bc_end).into_iter().map(|bc| Some(bc));
            iter.collect()
        };
        assert!(bcs.len() < 10_000); // TODO avoid magic number
        let n = wth.saturating_sub(bcs.len() as u16);
        let pad: Vec<Option<usize>> = iter::repeat(None).take(n).collect();
        bcs.extend(&pad);

        let cells {
            let iter = bcs.into_iter().zip((0..wth).into_iter());
            iter.map(|(bc, col)| Cell { bc, col, row }).collect()
        };
        Row { cells }
    }

    fn align(&self, bc: usize, cursor: Cursor) -> VShift {
        use std::cmp::Ordering::{Equal, Less, Greater};

        let iter = self.cells.iter().take_while(|cell| {
            let ok = cell.row < cursor.row;
            ok || (cell.row == cursor.row) && (cell.col <= cursor.col)
        });
        let iter = iter.rev().skip(|cell| cell.bc.is_none());

        match iter.first() {
            Some(cell { bc: Some(cell_bc), .. }) => match cell_bc.cmp(&bc) {
                Equal => VShift::None,
                Less => VShift::Left(bc-cell_bc),
                Greater => VShift::Right(cell_bc-bc),
            }
            None => VShift::None
        }
    }
}
