use crate::backend::Backend;
use crate::backend::ClearType;
use crate::buffer::Buffer;
use crate::layout::Position;
use crate::layout::Rect;
use crate::layout::Size;
use crate::terminal::frame::CompletedFrame;
use crate::terminal::Frame;
use crate::terminal::Viewport;

pub struct Options {
    /// Whether the terminal should respect CTRL+C
    respect_exit: bool,
    viewport: Viewport,
}

/// An abstraction over output through a given backend
pub struct Terminal<B: Backend> {
    backend: B,
    buffers: [Buffer; 2],
    /// Index of the current buffer
    current: usize,
    hidden_cursor: bool,
    viewport: Viewport,
    viewport_area: Rect,
    last_known_area: Rect,
    last_known_cursor_pos: Position,
}

impl<B: Backend> Terminal<B> {
    /// Create a new terminal with the given handle (implementing [Write]).
    /// Returns none if terminal width cannot be retrieved
    pub fn new(backend: B) -> Result<Self, B::Error> {
        Self::with_options(
            backend,
            Options {
                respect_exit: true,
                viewport: Viewport::Inline(1),
            },
        )
    }

    pub fn with_options(mut backend: B, options: Options) -> Result<Self, B::Error> {
        let area = match options.viewport {
            Viewport::Inline(height) => backend.size()?.into(),
            Viewport::Fixed(rect) => rect,
        };

        let (viewport_area, cursor_pos) = match options.viewport {
            Viewport::Inline(height) => {
                compute_inline_size(&mut backend, height, area.as_size(), 0)?
            }
            Viewport::Fixed(area) => (area, area.as_position()),
        };

        Ok(Self {
            backend,
            buffers: [Buffer::empty(viewport_area), Buffer::empty(viewport_area)],
            current: 0,
            hidden_cursor: false,
            viewport: options.viewport,
            viewport_area,
            last_known_area: area,
            last_known_cursor_pos: cursor_pos,
        })
    }

    pub const fn get_frame(&mut self) -> Frame<'_> {
        Frame {
            cursor_position: None,
            viewport_area: self.viewport_area,
            buffer: self.current_buffer_mut(),
        }
    }

    pub const fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }

    pub const fn backend(&self) -> &B {
        &self.backend
    }

    pub const fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Gets a diff between the current and previous buffers and passes it to the backend to be
    /// drawn
    pub fn flush(&mut self) -> Result<(), B::Error> {
        let previous_buffer = &self.buffers[1 - self.current];
        let current_buffer = &self.buffers[self.current];
        let updates = previous_buffer.diff(current_buffer);
        if let Some((col, row, _)) = updates.last() {
            self.last_known_cursor_pos = Position { x: *col, y: *row };
        }

        self.backend.draw(updates.into_iter())
    }

    pub fn resize(&mut self, area: Rect) -> Result<(), B::Error> {
        let next_area = match self.viewport {
            Viewport::Inline(height) => {
                let offset_in_previous_viewport = self
                    .last_known_cursor_pos
                    .y
                    .saturating_sub(self.viewport_area.top());
                compute_inline_size(
                    &mut self.backend,
                    height,
                    area.as_size(),
                    offset_in_previous_viewport,
                )?
                .0
            }
            Viewport::Fixed(_) => area,
        };

        self.set_viewport_area(next_area);
        self.clear()?;

        self.last_known_area = area;
        Ok(())
    }

    pub fn draw<F>(&mut self, render_callback: F) -> Result<CompletedFrame<'_>, B::Error>
    where
        F: FnOnce(&mut Frame),
    {
        self.try_draw(|frame| {
            render_callback(frame);
            Ok::<(), B::Error>(())
        })
    }

    pub fn try_draw<F, E>(&mut self, render_callback: F) -> Result<CompletedFrame<'_>, B::Error>
    where
        F: FnOnce(&mut Frame) -> Result<(), E>,
        E: Into<B::Error>,
    {
        self.autoresize()?;

        let mut frame = self.get_frame();

        render_callback(&mut frame).map_err(Into::into)?;

        let cursor_position = frame.cursor_position;

        self.flush()?;

        match cursor_position {
            None => self.hide_cursor()?,
            Some(position) => {
                self.show_cursor();
                self.set_cursor_position(position)?
            }
        }

        self.swap_buffers();

        self.backend.flush()?;

        let completed_frame = CompletedFrame {
            buffer: &self.buffers[1 - self.current],
            area: self.last_known_area,
        };

        Ok(completed_frame)
    }

    pub fn hide_cursor(&mut self) -> Result<(), B::Error> {
        self.backend.hide_cursor()?;
        self.hidden_cursor = true;
        Ok(())
    }

    pub fn show_cursor(&mut self) -> Result<(), B::Error> {
        self.backend.show_cursor()?;
        self.hidden_cursor = false;
        Ok(())
    }

    pub fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> Result<(), B::Error> {
        let position = position.into();
        self.backend.set_cursor_position(position)?;
        self.last_known_cursor_pos = position;
        Ok(())
    }

    pub fn get_cursor_position(&mut self) -> Result<Position, B::Error> {
        self.backend.get_cursor_position()
    }

    fn set_viewport_area(&mut self, area: Rect) {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].resize(area);
        self.viewport_area = area;
    }

    pub fn autoresize(&mut self) -> Result<(), B::Error> {
        if matches!(self.viewport, Viewport::Inline(_)) {
            let area = self.size()?.into();
            if area != self.last_known_area {
                self.resize(area)?;
            }
        }
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), B::Error> {
        match self.viewport {
            Viewport::Inline(_) => {
                self.backend
                    .set_cursor_position(self.viewport_area.as_position())?;
                self.backend.clear_region(ClearType::AfterCursor)?;
            }
            Viewport::Fixed(_) => {
                let area = self.viewport_area;
                for y in area.top()..area.bottom() {
                    self.backend.set_cursor_position(Position { x: 0, y })?;
                    self.backend.clear_region(ClearType::AfterCursor)?;
                }
            }
        }

        self.buffers[1 - self.current].reset();
        Ok(())
    }

    pub fn swap_buffers(&mut self) {
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;
    }

    pub fn size(&mut self) -> Result<Size, B::Error> {
        self.backend.size()
    }
}

/// Compute the size of the inline viewport

// This function comes from Ratatui, but isn't explained very well there. This version is highly
// commented to make it more comprehensible
fn compute_inline_size<B: Backend>(
    backend: &mut B,
    height: u16,
    size: Size,
    offset_in_previous_viewport: u16,
) -> Result<(Rect, Position), B::Error> {
    let pos = backend.get_cursor_position()?;
    let mut row = pos.y;

    let max_height = size.height.min(height);

    let lines_after_cursor = height
        .saturating_sub(offset_in_previous_viewport)
        .saturating_sub(1);

    backend.append_lines(lines_after_cursor)?;

    let available_lines = size.height.saturating_sub(row).saturating_sub(1);
    let missing_lines = lines_after_cursor.saturating_sub(available_lines);

    if missing_lines > 0 {
        row = row.saturating_sub(missing_lines);
    }

    row = row.saturating_sub(offset_in_previous_viewport);

    Ok((
        Rect {
            x: 0,
            y: row,
            width: size.width,
            height: max_height,
        },
        pos,
    ))
}
