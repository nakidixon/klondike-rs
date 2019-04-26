use std::io::Write;

use termion::{color, cursor, cursor::DetectCursorPos};

use super::{bounds::Bounds, coords::Coords, Result};

pub trait BlankPainter {
    fn draw_blank_excess(&mut self, bounds: Bounds) -> Result;
}

impl<W> BlankPainter for W
where
    W: Write,
{
    fn draw_blank_excess(&mut self, bounds: Bounds) -> Result {
        let color = color::Fg(color::Reset);
        write!(self, "{}", color)?;

        let width = bounds.width();
        let blank_line = " ".repeat(width as usize);

        for y in 0..bounds.height() {
            let line_coords = bounds.top_left + Coords::from_y(y);
            let goto: cursor::Goto = line_coords.into();
            write!(self, "{}{}", goto, blank_line)?;
        }

        debug!("Blanked {:?}", bounds);

        Ok(())
    }
}
