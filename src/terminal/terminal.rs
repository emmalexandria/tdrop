use crate::backend::Backend;
use crate::buffer::Buffer;
use crate::layout::Position;
use crate::layout::Rect;
use crate::layout::Size;
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
