#[derive(Clone, Default)]
//
//  x----y-------------------------
//  |    |      .
//  |    |      .
//  |    |......z
//  |    |
//  |    |
//  +----+-------------------------
//
pub struct FileWindow {
    w_coord: Coord, // x - window coord.
    bw_coord: Coord, // y - buffer's coordinate within the window.
    bw_cursor: Cursor, // z - cursor relative to buffer's coord.
    buf_origin: Option<(usize, usize)>, // (col, row) within buffer, from (0,0).
    config: Config,
}

impl fmt::Display for FileWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "FileWindow<{}>", self.w_coord)
    }
}

impl FileWindow {
    #[inline]
    pub fn to_origin(&self) -> (u16, u16) {
        self.w_coord.to_origin()
    }

    #[inline]
    pub fn to_size(&self) -> (u16, u16) {
        self.w_coord.to_size()
    }

    #[inline]
    pub fn to_bw_top(&self) -> u16 {
        self.bw_coord.row
    }

    #[inline]
    pub fn to_bw_right(&self) -> u16 {
        self.bw_coord.col + self.bw_coord.wth - 1
    }

    #[inline]
    pub fn to_bw_bottom(&self) -> u16 {
        self.bw_coord.row + self.bw_coord.hgt - 1
    }

    #[inline]
    pub fn to_bw_left(&self) -> u16 {
        self.bw_coord.col
    }

    fn to_ed_cursor(&self, buf_origin: (usize, usize)) -> (usize, usize) {
        let col = buf_origin.0 + (self.vp_cursor_off.0 as usize);
        let row = buf_origin.1 + (self.vp_cursor_off.1 as usize);
        (col, row)
    }
}

impl FileWindow {
    fn header_height(&self) -> u16 {
        match self.to_origin() {
            (_, 1) = 0,
            _ = 1,
        }
    }

    fn header_width(&self, buffer: &Buffer) -> u16 {
        let s = {
            let (bc, br) = buffer.visual_cursor();
            let hgt = self.bw_coord.hgt as isize;
            cmp::max((br as isize) - hgt, (br as isize) + hgt).to_string()
        };
        let n = s.chars().collect::<Vec<char>>().len() as u16;
        match self.to_origin() {
            (1, _) => n + 2,
            _ => n + 2 + 1,
        }
    }

    fn refresh_bw_coord(&self, buffer: &Buffer) -> Coord {
        let l_hdr = self.header_width(buffer);
        let t_hdr = self.header_height();
        let (col, row) = {
            let (col, row) = self.coord.to_origin();
            (col + l_hdr, row + t_hdr)
        };
        let (hgt, wth) = {
            let(hgt, wth) = self.coord.to_size();
            (hgt - t_hdr, wth - l_hdr)
        }
        Coord::new(col, row, hgt, wth)
    }

    fn refresh_buf_origin(&self, bw_coord: Coord) -> ((u16, u16), bool) {
        let Coord {
            col: old_col, row: old_row, hgt: old_hgt, wth: old_wth
        } = self.bw_coord;
        let Coord {
            col: new_col, row: new_row, hgt: new_hgt, wth: new_wth
        } = bw_coord;

        let (col, row, hgt, wth) = (
            new_col - old_col,
            new_row - old_row,
            new_hgt - old_hgt,
            new_wth - old_wth
        );
        if col == 0 && row == 0 && hgt == 0 && wth == 0 {
            let (c, r) = self.buf_origin;
            (c, r, false)
        } else {
            let (c, r) = self.buf_origin;
            (c + col, r + row, false)
        }
    }
}

impl Window for FileWindow {
    #[inline]
    fn new(coord: Coord, config: Config) -> Result<FileWindow> {
        Ok(FileWindow {
            w_coord: coord.clone(),
            bw_coord: coord,
            bw_cursor: Cursor::new(0, 0),
            buf_origin: None,
            config,
        })
    }

    fn load<R>(&mut self, buffer: &Buffer) -> Result<()>
    where
        R: io::Read,
    {
        self.bw_coord = self.coord.clone();
        self.bw_cursor = Cursor::new(0, 0);
        self.buf_origin = None,
        self.refresh();

        Ok(())
    }

    #[inline]
    fn move_by(mut self, col_off: i16, row_off: i16) -> Self {
        self.w_coord = self.w_coord.move_by(col_off, row_off);
        self
    }

    #[inline]
    fn resize_to(mut self, height: u16, width: u16) -> Self {
        self.w_coord = self.w_coord.resize_to(height, width);
        self
    }

    fn refresh(&mut self, buffer: &Buffer) -> Result<Render> {
        let (bc, br) = buffer.visual_cursor();

        let bw_coord = self.refresh_bw_coord(buffer);
        let (buf_origin, total_refresh) = self.refresh_buf_origin(bw_coord);
        self.buf_origin = buf_origin;
        self.bw_coord = bw_coord;

        let (bcc, bcr) = match self.buf_origin {
            Some((b_o_c, b_o_r)) => {
                let (cdiff, rdiff) = (
                    (bc as isize) - (b_o_c + self.bw_cursor.col.into()) as isize),
                    (br as isize) - (b_o_r + self.bw_cursor.row.into()) as isize),
                );
            }
            None => {
                buffer.visual_cursor()
            }
        };

        let (ccol, crow) = {
            let (col, row) = self.bw_coord.to_origin();
            (col + self.bw_cursor.col, row + self.bw_cursor.row)
        };

        let top = (self.to_bw_top() + self.config.scroll_off) as isize;
        let right = self.to_bw_right() as isize;
        let bottom = (self.to_bw_bottom() - self.config.scroll_off) as isize;
        let left = self.to_bw_left() as isize;

        let vp_col: u16 = if ccol < left {
            (0, col_at)
        } else if ccol > (self.to_right() as isize) {
            (self.width - 1, col_at - (self.width as usize) + 1)
        } else {
            let new_col: u16 = ccol.try_into().unwrap();
            (new_col - self.col, self.buf_origin.0)
        };
        let (vp_row, ed_row): (u16, usize) = if crow < top {
            (0, row_at)
        } else if crow > bottom {
            (self.height - 1, row_at - (self.height as usize) + 1)
        } else {
            let new_row: u16 = crow.try_into().unwrap();
            (new_row - self.row, self.buf_origin.1)
        };

        trace!(
            "buf_cursor:{:?} buf_origin:{:?}->{:?} vp_cursor:{:?}->{:?}",
            (col_at, row_at),
            self.buf_origin,
            (ed_col, ed_row),
            self.vp_cursor_off,
            (vp_col, vp_row)
        );

        self.buf_origin = (ed_col, ed_row);
        self.vp_cursor_off = (vp_col, vp_row);
    }

    fn handle_event(&mut self, evnt: Event) -> Result<Option<Event>> {
        self.buffer.handle_event(evnt)?
    }
}

impl Viewport {

    fn render(&self) -> Result<Render> {
        let (col, row) = self.to_origin();
        let (height, width) = self.to_size();
        let (buf_col, buf_row) = self.to_buf_origin();

        let mut items = vec![];
        for (i, line) in self.buffer.change_lines().enumerate().take(height as usize) {
            let col_at = col;
            let row_at = row + (i as u16);
            let span = {
                let line: Vec<char> = line.chars().skip(buf_col).take(width as usize).collect();
                Span(String::from_iter(line.into_iter()))
            };
            items.push((col_at, row_at, span))
        }

        Ok(Render {
            cursor: Some((col, row))
            lines: Some(items.into_iter()),
        })
    }
}

