#[derive(Clone)]
pub struct Wrap {
    name: String,
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    nu: ColNu,
    scroll_off: u16,
    line_number: bool,
    screen_lines: Vec<ScrLine>,
}

impl fmt::Display for Wrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "Wrap<{:?} {} {} {}@{} {}>",
            self.name,
            self.nu,
            self.obc_xy,
            self.cursor,
            self.coord,
            self.screen_lines.len()
        )
    }
}

impl<'a, W> TryFrom<(&'a W, buffer::Cursor)> for Wrap
where
    W: Window,
{
    type Error = Error;

    fn try_from((w, obc_xy): (&'a W, buffer::Cursor)) -> Result<Wrap> {
        let cursor = {
            let e = Error::Invalid(String::default(), "no-cursor".to_string());
            err_at!(w.to_cursor().ok_or(e))?
        };
        let scroll_off = w.config_scroll_offset();
        let line_number = w.config_line_number();
        let mut value = Wrap {
            name: w.to_name(),
            coord: w.to_coord(),
            cursor,
            obc_xy,
            nu: ColNu::new(obc_xy.row, line_number),
            scroll_off,
            line_number,
            screen_lines: Vec::default(),
        };
        value.discount_nu(ColNu::new(obc_xy.row, line_number).to_width());
        Ok(value)
    }
}
