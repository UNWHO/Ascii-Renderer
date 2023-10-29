use crossterm::queue;
use std::io::{self, Stdout, Write};

use crate::Color;
pub struct FrameBuffer<'a> {
    width: usize,
    height: usize,

    buf: Vec<Vec<char>>,

    w: &'a mut Stdout,
}

impl<'a> FrameBuffer<'a> {
    pub fn new(width: usize, height: usize, stdout: &'a mut Stdout) -> Self {
        FrameBuffer {
            width,
            height,
            buf: vec![vec![' '; width]; height],
            w: stdout,
        }
    }

    pub fn print(&mut self) -> io::Result<()> {
        queue!(
            self.w,
            crossterm::cursor::MoveTo(1, 1),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        )?;

        for i in 0..self.height {
            for j in 0..self.width {
                queue!(self.w, crossterm::style::Print(self.buf[i][j]))?;
            }
            queue!(self.w, crossterm::cursor::MoveToNextLine(1))?;
        }

        self.w.flush()?;

        Ok(())
    }

    pub fn clear(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.buf[i][j] = ' ';
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, _color: Color) {
        self.buf[y][x] = '*';
    }
}
