use std::ops::{Index, IndexMut};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::cell::Cell,
    layout::{position::Position, rect::Rect},
    Style,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Buffer {
    content: Vec<Cell>,
    area: Rect,
}

impl Buffer {
    pub fn empty(area: Rect) -> Self {
        Self::filled(area, Cell::empty())
    }

    pub fn filled(area: Rect, cell: Cell) -> Self {
        let count = area.area();

        Self {
            area,
            content: vec![cell; count as usize],
        }
    }

    #[must_use]
    pub fn index_of(&self, x: u16, y: u16) -> usize {
        (y as usize * self.area.width as usize) + x as usize
    }

    pub fn set_string<T, S>(&mut self, x: u16, y: u16, string: T, style: S)
    where
        T: AsRef<str>,
        S: Into<Style>,
    {
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
        let mut remaining_width = self.area.right().saturating_sub(x).min(max_width);
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

    pub fn render(&self) -> String {
        String::new()
    }
}

impl<P: Into<Position>> Index<P> for Buffer {
    type Output = Cell;

    fn index(&self, position: P) -> &Self::Output {
        let position = position.into();
        &self.content[self.index_of(position.x, position.y)]
    }
}

impl<P: Into<Position>> IndexMut<P> for Buffer {
    fn index_mut(&mut self, position: P) -> &mut Self::Output {
        let position = position.into();
        let index = self.index_of(position.x, position.y);
        &mut self.content[index]
    }
}
