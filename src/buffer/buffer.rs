use std::{
    cmp,
    ops::{Index, IndexMut},
};

use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Cell,
    layout::{Position, Rect},
    style::Style,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Buffer {
    pub area: Rect,
    pub content: Vec<Cell>,
}

impl Buffer {
    #[must_use]
    pub fn empty(area: Rect) -> Self {
        Self::filled(area, Cell::EMPTY)
    }

    #[must_use]
    pub fn filled(area: Rect, cell: Cell) -> Self {
        let size = area.area() as usize;
        let content = vec![cell; size];
        Self { area, content }
    }

    pub fn pos_of(&self, index: usize) -> (u16, u16) {
        let x = index % self.area.width as usize + self.area.x as usize;
        let y = index / self.area.width as usize + self.area.y as usize;
        (
            u16::try_from(x).expect("Oopsies"),
            u16::try_from(y).expect("Oopsies"),
        )
    }

    pub fn index_of(&self, x: u16, y: u16) -> usize {
        self.index_of_opt(Position { x, y }).unwrap_or_else(|| {
            panic!(
                "index outside of buffer: the area is {area:?} but index is {x}, {y}",
                area = self.area
            )
        })
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

    pub fn reset(&mut self) {
        for cell in &mut self.content {
            cell.reset();
        }
    }

    pub fn diff<'a>(&self, other: &'a Self) -> Vec<(u16, u16, &'a Cell)> {
        let previous_buffer = &self.content;
        let next_buffer = &other.content;

        let mut updates: Vec<(u16, u16, &Cell)> = vec![];

        let mut invalidated: usize = 0;

        let mut to_skip: usize = 0;
        for (i, (current, previous)) in next_buffer.iter().zip(previous_buffer.iter()).enumerate() {
            if !current.skip && (current != previous || invalidated > 0) && to_skip == 0 {
                let (x, y) = self.pos_of(i);
                updates.push((x, y, &next_buffer[i]))
            }

            to_skip = current.symbol().width().saturating_sub(1);

            let affected_width = cmp::max(current.symbol().width(), previous.symbol().width());
            invalidated = cmp::max(affected_width, invalidated).saturating_sub(1)
        }

        updates
    }

    pub fn set_style<S: Into<Style>>(&mut self, area: Rect, style: S) {
        let style = style.into();
        let area = self.area.intersection(area);

        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                self[(x, y)].set_style(style);
            }
        }
    }

    pub fn resize(&mut self, area: Rect) {
        let length = area.area() as usize;
        if self.content.len() > length {
            self.content.truncate(length);
        } else {
            self.content.resize(length, Cell::EMPTY)
        }

        self.area = area;
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
