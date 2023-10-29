use crossterm::{execute, queue};
use std::io::{self, Stdout, Write};

use crate::{Color, Fragment};
pub struct FrameBuffer<'a> {
    width: usize,
    height: usize,

    buf: Vec<Vec<Option<Fragment>>>,

    w: &'a mut Stdout,
}

impl<'a> FrameBuffer<'a> {
    pub fn new(width: usize, height: usize, stdout: &'a mut Stdout) -> Self {
        execute!(
            stdout,
            // crossterm::terminal::SetSize((width + 1) as u16, (height + 1) as u16),
            crossterm::cursor::Hide
        )
        .expect("Failed to set size of screen buffer");

        FrameBuffer {
            width,
            height,
            buf: vec![vec![None; width]; height],
            w: stdout,
        }
    }

    pub fn print(&mut self) -> io::Result<()> {
        queue!(
            self.w,
            crossterm::cursor::MoveTo(1, 1),
            // crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge),
        )?;

        for i in 0..self.height {
            for j in 0..self.width {
                let fragment = &self.buf[i][j];

                match fragment {
                    None => queue!(self.w, crossterm::style::Print(' '))?,
                    Some(fragment) => {
                        queue!(
                            self.w,
                            crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
                                r: fragment.color.r,
                                g: fragment.color.g,
                                b: fragment.color.b
                            }),
                            crossterm::style::Print(bright_to_ascii(&fragment.color))
                        )?;
                    }
                }
            }
            queue!(self.w, crossterm::cursor::MoveToNextLine(1))?;
        }

        self.w.flush()?;

        Ok(())
    }

    pub fn clear(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.buf[i][j] = None;
            }
        }
    }

    pub fn set_pixel(&mut self, fragment: Fragment) {
        let x = fragment.x;
        let y = fragment.y;

        self.buf[y][x] = Some(fragment);
    }
}

const ASCII: &str = ".:-=+*#%@";
fn bright_to_ascii(color: &Color) -> char {
    let i = (color.lightness() * ASCII.len() as f64).floor() as usize;
    return ASCII.as_bytes()[8] as char;
}
