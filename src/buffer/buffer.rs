use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Cell,
    layout::{position::Position, rect::Rect},
    Style,
};

/** Buffer is the output type in tdrop, and is what widgets return */
#[derive(Debug, Clone, Default)]
pub struct Buffer {
    area: Rect,
    content: Vec<Cell>,
}

impl Buffer {
    #[must_use]
    pub fn empty(area: Rect) -> Self {
        Self::filled(area, Cell::empty())
    }

    #[must_use]
    pub fn filled(area: Rect, cell: Cell) -> Self {
        let size = area.area() as usize;
        let cells = vec![cell; size];
        Self {
            area,
            content: cells,
        }
    }

    pub fn index_of(&self, x: u16, y: u16) -> usize {
        self.index_of_opt(Position { x, y }).unwrap_or_else(|| {
            panic!(
                "index outside of buffer: the area is {area:?} but index is ({x}, {y})",
                area = self.area,
            )
        })
    }

    #[must_use]
    const fn index_of_opt(&self, position: Position) -> Option<usize> {
        let area = self.area;

        Some((position.y * area.width + position.x) as usize)
    }

    pub fn set_string<T, S>(&mut self, x: u16, y: u16, string: T, style: S)
    where
        T: AsRef<str>,
        S: Into<Style>,
    {
        self.set_stringn(x, y, string, usize::MAX, style);
    }

    /// Print at most the first n characters of a string if enough space is available
    /// until the end of the line. Skips zero-width graphemes and control characters.
    ///
    /// Use [`Buffer::set_string`] when the maximum amount of characters can be printed.
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

    pub fn display(&self) -> String {
        let mut output: Vec<String> = vec![String::new()];
        let mut curr_x = 0;
        let mut curr_y = 0;
        let mut content = self.content.clone();

        while content.len() > 0 {
            while curr_x <= self.area.width {
                if curr_x == self.area.width {
                    curr_y += 1;
                    curr_x = 0;
                    output.push(String::new());
                    break;
                }

                let cell = &self[(curr_x, curr_y)];

                output[curr_y as usize].push_str(&cell.display());
                curr_x += 1;
                content.remove(0);
            }
        }
        output = output.iter().filter(|s| !s.is_empty()).cloned().collect();
        output.join("\n")
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display())
    }
}

impl<P: Into<Position>> Index<P> for Buffer {
    type Output = Cell;

    fn index(&self, position: P) -> &Self::Output {
        let position = position.into();
        let idx = self.index_of(position.x, position.y);
        &self.content[idx]
    }
}

impl<P: Into<Position>> IndexMut<P> for Buffer {
    fn index_mut(&mut self, position: P) -> &mut Self::Output {
        let position = position.into();
        let idx = self.index_of(position.x, position.y);
        &mut self.content[idx]
    }
}

#[cfg(test)]
mod test {
    use crate::{
        buffer::{Buffer, Cell},
        layout::rect::Rect,
    };

    #[test]
    fn display() {
        let area = Rect::new(0, 0, 5, Some(5));
        let mut cell = Cell::empty();
        cell.set_symbol("h");
        let expected = "hhhhh\nhhhhh\nhhhhh\nhhhhh\nhhhhh";

        let buffer = Buffer::filled(area, cell);

        assert_eq!(buffer.display(), expected);
    }
}
