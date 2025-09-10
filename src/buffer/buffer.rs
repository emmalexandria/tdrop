use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::cell::{self, Cell},
    layout::{position::Position, rect::Rect},
    Style,
};

#[derive(Debug, Clone)]
pub struct Buffer {
    pub area: Rect,

    content: Vec<Cell>,
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl Buffer {
    pub fn new(area: &Rect) -> Self {
        let content = vec![Cell::empty(); (area.width as usize * area.height as usize)];
        Self {
            area: *area,
            content,
        }
    }

    pub fn set_cell(x: u16, y: u16, cell: Cell) {}

    pub fn index_of(&self, x: u16, y: u16) -> usize {
        self.index_of_opt(Position { x, y })
            .unwrap_or_else(|| panic!("index outside of buffer"))
    }

    const fn index_of_opt(&self, position: Position) -> Option<usize> {
        let area = self.area;
        if !area.contains(position) {
            return None;
        }

        let y = (position.y - self.area.y) as usize;
        let x = (position.x - self.area.x) as usize;
        let width = self.area.width as usize;
        Some(y * width + x)
    }

    pub fn set_string<T, S>(&mut self, x: u16, y: u16, string: T, style: S)
    where
        T: AsRef<str>,
        S: Into<Style>,
    {
        self.set_stringn(x, y, string, usize::MAX, style);
    }

    pub fn set_stringn<T, S>(
        &mut self,
        mut x: u16,
        y: u16,
        string: T,
        max_width: usize,
        style: S,
    ) -> (u16, u16)
    where
        T: AsRef<str>,
        S: Into<Style>,
    {
        let max_width = max_width.try_into().unwrap_or(u16::MAX);
        let mut remaining_width = self.area.right();

        let graphemes = UnicodeSegmentation::graphemes(string.as_ref(), true)
            .filter(|symbol| !symbol.contains(char::is_control))
            .map(|symbol| (symbol, symbol.width() as u16))
            .filter(|(_symbol, width)| *width > 0)
            .map_while(|(symbol, width)| {
                remaining_width = remaining_width.checked_sub(width)?;
                Some((symbol, width))
            });

        let style = style.into();
        for (symbol, width) in graphemes {
            self[(x, y)].set_symbol(symbol).set_style(style);
            let next_symbol = x + width;
            x += 1;
            // Reset following cells if multi-width (they would be hidden by the grapheme),
            while x < next_symbol {
                self[(x, y)].reset();
                x += 1;
            }
        }
        (x, y)
    }

    pub fn display(&self) -> String {
        let mut lines: Vec<String> = (0..self.area.height).map(|_| String::new()).collect();
        let mut curr_x = 0;
        let mut curr_y = 0;

        while curr_y < lines.len() {
            while curr_x < self.area.width {
                if curr_x == self.area.width - 1 {
                    curr_y += 1;
                    break;
                }

                let cell_idx = (self.area.width as usize * curr_y) + curr_x as usize;

                lines[curr_y].push_str(&self.content[cell_idx].to_string());
                curr_x += 1;
            }
        }

        return lines.join("\n");
    }
}

impl<P: Into<Position>> Index<P> for Buffer {
    type Output = Cell;

    fn index(&self, position: P) -> &Self::Output {
        let position = position.into();
        let index = self.index_of(position.x, position.y);
        &self.content[index]
    }
}

impl<P: Into<Position>> IndexMut<P> for Buffer {
    fn index_mut(&mut self, position: P) -> &mut Self::Output {
        let position = position.into();
        let index = self.index_of(position.x, position.y);
        &mut self.content[index]
    }
}
