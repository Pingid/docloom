//! Terminal renderer with ANSI color support.
//!
//! The `term` module renders [`Block`] trees into strings that
//! include indentation, Unicode box drawing, and configurable ANSI styling.
//!
//! # Examples
//! ```rust
//! use docloom::prelude::*;
//! use docloom::term::{Style, doc};
//!
//! // Optional style configuration
//! let style = Style::plain()
//!     .unicode_boxes(false)
//!     .colors(false)
//!     .heading_colors([
//!         Style::BRIGHT_CYAN,
//!         Style::CYAN,
//!         Style::BRIGHT_BLUE,
//!         Style::BLUE,
//!         Style::BRIGHT_WHITE,
//!         Style::BRIGHT_WHITE,
//!     ]);
//!
//! let rendered = doc([
//!     h1("Terminal Output"),
//!     p("Render ANSI-enhanced text from structured blocks."),
//! ])
//! .with_style(style)
//! .to_string();
//! ```

use itemize::IntoItems;
use std::fmt;

use super::{Alignment, Block, Inline, Render, Renderable};

/// Terminal document wrapper that renders blocks with terminal [`Style`].
pub struct Doc {
    content: Vec<Block>,
    style: Style,
}

impl Doc {
    /// Create a document from values convertible to [`Block`].
    pub fn new(value: impl IntoItems<Block>) -> Self {
        Self {
            content: value.into_items().collect(),
            style: Style::default(),
        }
    }

    /// Override the rendering style to use when formatting the document.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl fmt::Display for Doc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.content
            .render_with(&mut Renderer::with_style(f, self.style))
    }
}

/// Construct a [`Doc`] from any value that can become a sequence of blocks.
pub fn doc(value: impl IntoItems<Block>) -> Doc {
    Doc::new(value)
}

/// Configuration for terminal rendering style.
#[derive(Clone, Copy)]
pub struct Style {
    /// Whether to emit ANSI color escape codes.
    pub use_colors: bool,
    /// Whether to rely on Unicode box drawing characters.
    pub use_unicode_boxes: bool,
    /// Colors to cycle through for heading levels.
    pub heading_colors: [&'static str; 6],
    /// Foreground color used for code blocks.
    pub code_color: &'static str,
    /// Background color used for code blocks.
    pub code_bg: &'static str,
    /// Color used for rendering hyperlinks.
    pub link_color: &'static str,
    /// Color used for list markers.
    pub list_color: &'static str,
    /// Color used for borders such as code block boxes.
    pub border_color: &'static str,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            use_colors: true,
            use_unicode_boxes: true,
            heading_colors: [
                Style::BRIGHT_CYAN,
                Style::CYAN,
                Style::BRIGHT_BLUE,
                Style::BLUE,
                Style::BRIGHT_WHITE,
                Style::BRIGHT_WHITE,
            ],
            code_color: Style::GREEN,
            code_bg: Style::BG_BLACK,
            link_color: Style::BRIGHT_BLUE,
            list_color: Style::BRIGHT_YELLOW,
            border_color: Style::BRIGHT_BLACK,
        }
    }
}

impl Style {
    /// ANSI escape code for a black foreground.
    pub const BLACK: &str = "\x1b[30m";
    /// ANSI escape code for a red foreground.
    pub const RED: &str = "\x1b[31m";
    /// ANSI escape code for a green foreground.
    pub const GREEN: &str = "\x1b[32m";
    /// ANSI escape code for a yellow foreground.
    pub const YELLOW: &str = "\x1b[33m";
    /// ANSI escape code for a blue foreground.
    pub const BLUE: &str = "\x1b[34m";
    /// ANSI escape code for a magenta foreground.
    pub const MAGENTA: &str = "\x1b[35m";
    /// ANSI escape code for a cyan foreground.
    pub const CYAN: &str = "\x1b[36m";
    /// ANSI escape code for a white foreground.
    pub const WHITE: &str = "\x1b[37m";

    /// ANSI escape code for a bright black foreground.
    pub const BRIGHT_BLACK: &str = "\x1b[90m";
    /// ANSI escape code for a bright red foreground.
    pub const BRIGHT_RED: &str = "\x1b[91m";
    /// ANSI escape code for a bright green foreground.
    pub const BRIGHT_GREEN: &str = "\x1b[92m";
    /// ANSI escape code for a bright yellow foreground.
    pub const BRIGHT_YELLOW: &str = "\x1b[93m";
    /// ANSI escape code for a bright blue foreground.
    pub const BRIGHT_BLUE: &str = "\x1b[94m";
    /// ANSI escape code for a bright magenta foreground.
    pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
    /// ANSI escape code for a bright cyan foreground.
    pub const BRIGHT_CYAN: &str = "\x1b[96m";
    /// ANSI escape code for a bright white foreground.
    pub const BRIGHT_WHITE: &str = "\x1b[97m";

    /// ANSI escape code for a black background.
    pub const BG_BLACK: &str = "\x1b[40m";
    /// ANSI escape code for a red background.
    pub const BG_RED: &str = "\x1b[41m";
    /// ANSI escape code for a green background.
    pub const BG_GREEN: &str = "\x1b[42m";
    /// ANSI escape code for a yellow background.
    pub const BG_YELLOW: &str = "\x1b[43m";
    /// ANSI escape code for a blue background.
    pub const BG_BLUE: &str = "\x1b[44m";
    /// ANSI escape code for a magenta background.
    pub const BG_MAGENTA: &str = "\x1b[45m";
    /// ANSI escape code for a cyan background.
    pub const BG_CYAN: &str = "\x1b[46m";
    /// ANSI escape code for a white background.
    pub const BG_WHITE: &str = "\x1b[47m";

    /// ANSI escape code for a bright black background.
    pub const BG_BRIGHT_BLACK: &str = "\x1b[100m";
    /// ANSI escape code for a bright red background.
    pub const BG_BRIGHT_RED: &str = "\x1b[101m";
    /// ANSI escape code for a bright green background.
    pub const BG_BRIGHT_GREEN: &str = "\x1b[102m";
    /// ANSI escape code for a bright yellow background.
    pub const BG_BRIGHT_YELLOW: &str = "\x1b[103m";
    /// ANSI escape code for a bright blue background.
    pub const BG_BRIGHT_BLUE: &str = "\x1b[104m";
    /// ANSI escape code for a bright magenta background.
    pub const BG_BRIGHT_MAGENTA: &str = "\x1b[105m";
    /// ANSI escape code for a bright cyan background.
    pub const BG_BRIGHT_CYAN: &str = "\x1b[106m";
    /// ANSI escape code for a bright white background.
    pub const BG_BRIGHT_WHITE: &str = "\x1b[107m";

    /// ANSI escape code to reset all text attributes.
    pub const RESET: &str = "\x1b[0m";
    /// ANSI escape code to enable bold text.
    pub const BOLD: &str = "\x1b[1m";
    /// ANSI escape code to dim text.
    pub const DIM: &str = "\x1b[2m";
    /// ANSI escape code to italicize text.
    pub const ITALIC: &str = "\x1b[3m";
    /// ANSI escape code to underline text.
    pub const UNDERLINE: &str = "\x1b[4m";
    /// ANSI escape code to strike through text.
    pub const STRIKETHROUGH: &str = "\x1b[9m";

    /// Create a style with no colors (plain text).
    pub fn plain() -> Self {
        Self {
            use_colors: false,
            ..Default::default()
        }
    }

    /// Create a style without Unicode box-drawing characters.
    pub fn ascii() -> Self {
        Self {
            use_unicode_boxes: false,
            ..Default::default()
        }
    }

    /// Enable or disable ANSI colors.
    pub fn colors(mut self, use_colors: bool) -> Self {
        self.use_colors = use_colors;
        self
    }

    /// Enable or disable Unicode box-drawing characters.
    pub fn unicode_boxes(mut self, use_unicode_boxes: bool) -> Self {
        self.use_unicode_boxes = use_unicode_boxes;
        self
    }

    /// Override the set of heading colors.
    pub fn heading_colors(mut self, heading_colors: [&'static str; 6]) -> Self {
        self.heading_colors = heading_colors;
        self
    }

    /// Set the code foreground color.
    pub fn code_color(mut self, code_color: &'static str) -> Self {
        self.code_color = code_color;
        self
    }

    /// Set the code background color.
    pub fn code_bg(mut self, code_bg: &'static str) -> Self {
        self.code_bg = code_bg;
        self
    }

    /// Set the hyperlink color.
    pub fn link_color(mut self, link_color: &'static str) -> Self {
        self.link_color = link_color;
        self
    }

    /// Set the list marker color.
    pub fn list_color(mut self, list_color: &'static str) -> Self {
        self.list_color = list_color;
        self
    }

    /// Set the border color used for code blocks and tables.
    pub fn border_color(mut self, border_color: &'static str) -> Self {
        self.border_color = border_color;
        self
    }
}

pub struct Renderer<'a, W> {
    writer: &'a mut W,
    indent_level: usize,
    style: Style,
}

impl<'a, W> Renderer<'a, W> {
    /// Create a renderer with default [`Style`] settings.
    pub fn new(writer: &'a mut W) -> Self {
        Self::with_style(writer, Style::default())
    }

    /// Create a renderer that uses a custom [`Style`].
    pub fn with_style(writer: &'a mut W, style: Style) -> Self {
        Self {
            writer,
            indent_level: 0,
            style,
        }
    }

    fn write_indent(&mut self) -> fmt::Result
    where
        W: fmt::Write,
    {
        write!(self.writer, "{}", "  ".repeat(self.indent_level))
    }

    fn color(&self, code: &'static str) -> &'static str {
        if self.style.use_colors { code } else { "" }
    }
}

impl Renderer<'_, String> {
    /// Render a value to a [`String`] using the default [`Style`].
    pub fn to_string<R>(r: &R) -> String
    where
        R: for<'b> Renderable<Renderer<'b, String>, Output = Result<(), fmt::Error>> + ?Sized,
    {
        Self::to_string_with_style(r, Style::default())
    }

    /// Render a value to a [`String`] using a custom [`Style`].
    pub fn to_string_with_style<R>(r: &R, style: Style) -> String
    where
        R: for<'b> Renderable<Renderer<'b, String>, Output = Result<(), fmt::Error>> + ?Sized,
    {
        let mut buf = String::new();
        r.render_with(&mut Renderer::with_style(&mut buf, style))
            .unwrap();
        buf
    }
}

impl<'a, W: fmt::Write> Render for Renderer<'a, W> {
    type Output = Result<(), fmt::Error>;

    /// Render a [`Block`] as colored terminal output.
    fn render_block(&mut self, inner: &Block) -> Self::Output {
        use Block::*;

        match inner {
            Paragraph(content) => {
                self.write_indent()?;
                content.render_with(self)?;
                writeln!(self.writer, "{}", self.color(Style::RESET))
            }

            Heading { level, content } => {
                writeln!(self.writer)?;
                self.write_indent()?;
                let level_idx = (*level as usize - 1).min(5);
                let color = self.style.heading_colors[level_idx];
                let prefix = if self.style.use_unicode_boxes {
                    ["█ ", "▓ ", "▒ ", "░ ", "• ", "• "][level_idx]
                } else {
                    ["# ", "## ", "### ", "#### ", "##### ", "###### "][level_idx]
                };

                write!(
                    self.writer,
                    "{}{}{}",
                    self.color(Style::BOLD),
                    self.color(color),
                    prefix
                )?;
                content.render_with(self)?;
                writeln!(self.writer, "{}", self.color(Style::RESET))?;
                writeln!(self.writer)
            }

            CodeBlock { language, content } => {
                writeln!(self.writer)?;
                self.write_indent()?;

                let (top, left, bottom) = if self.style.use_unicode_boxes {
                    ("┌─", "│", "└─")
                } else {
                    ("+--", "|", "+--")
                };

                // Top border
                if let Some(lang) = language {
                    writeln!(
                        self.writer,
                        "{}{}{}[ {} ]{}",
                        self.color(Style::DIM),
                        self.color(self.style.border_color),
                        top,
                        lang,
                        self.color(Style::RESET)
                    )?;
                } else {
                    writeln!(
                        self.writer,
                        "{}{}{}────{}",
                        self.color(Style::DIM),
                        self.color(self.style.border_color),
                        top,
                        self.color(Style::RESET)
                    )?;
                }

                // Content
                for line in content.to_string().lines() {
                    self.write_indent()?;
                    writeln!(
                        self.writer,
                        "{}{}{}{} {}{}",
                        self.color(Style::DIM),
                        self.color(self.style.border_color),
                        left,
                        self.color(Style::RESET),
                        self.color(self.style.code_color),
                        line
                    )?;
                }

                // Bottom border
                self.write_indent()?;
                writeln!(
                    self.writer,
                    "{}{}{}────{}",
                    self.color(Style::DIM),
                    self.color(self.style.border_color),
                    bottom,
                    self.color(Style::RESET)
                )?;
                writeln!(self.writer)
            }

            List { ordered, items } => {
                for (idx, item) in items.iter().enumerate() {
                    self.write_indent()?;
                    let marker = match (ordered, self.style.use_unicode_boxes) {
                        (true, _) => format!("{}.", idx + 1),
                        (false, true) => String::from("•"),
                        (false, false) => String::from("*"),
                    };
                    write!(
                        self.writer,
                        "{}{} {}",
                        self.color(self.style.list_color),
                        marker,
                        self.color(Style::RESET)
                    )?;

                    // self.indent_level += 1;
                    item.render_with(self)?;
                    // self.indent_level -= 1;
                }
                writeln!(self.writer)
            }

            TaskList { items } => {
                for (checked, item) in items.iter() {
                    self.write_indent()?;
                    let box_char = if self.style.use_unicode_boxes {
                        if *checked { "☑" } else { "☐" }
                    } else if *checked {
                        "[x]"
                    } else {
                        "[ ]"
                    };

                    write!(
                        self.writer,
                        "{}{} {}",
                        self.color(self.style.list_color),
                        box_char,
                        self.color(Style::RESET)
                    )?;

                    // self.indent_level += 1;
                    item.render_with(self)?;
                    // self.indent_level -= 1;
                }
                writeln!(self.writer)
            }

            Table {
                headers,
                rows,
                alignments,
            } => {
                // Calculate column widths
                let col_count = headers.len();
                let mut widths: Vec<usize> =
                    headers.iter().map(|h| Self::measure_inline(h)).collect();

                for row in rows {
                    for (i, cell) in row.iter().enumerate().take(col_count) {
                        widths[i] = widths[i].max(Self::measure_inline(cell));
                    }
                }

                for w in &mut widths {
                    *w = (*w).max(3);
                }

                let (tl, tr, bl, br, h, v, cross, t_down, t_up, t_left, t_right) =
                    if self.style.use_unicode_boxes {
                        ("┌", "┐", "└", "┘", "─", "│", "┼", "┬", "┴", "├", "┤")
                    } else {
                        ("+", "+", "+", "+", "-", "|", "+", "+", "+", "+", "+")
                    };

                // Top border
                self.write_indent()?;
                write!(
                    self.writer,
                    "{}{}{}",
                    self.color(Style::DIM),
                    self.color(self.style.border_color),
                    tl
                )?;
                for (i, w) in widths.iter().enumerate() {
                    write!(self.writer, "{}", h.repeat(w + 2))?;
                    if i < widths.len() - 1 {
                        write!(self.writer, "{}", t_down)?;
                    }
                }
                writeln!(self.writer, "{}{}", tr, self.color(Style::RESET))?;

                // Header row
                self.write_indent()?;
                write!(
                    self.writer,
                    "{}{}{}{}",
                    self.color(Style::DIM),
                    self.color(self.style.border_color),
                    v,
                    self.color(Style::RESET)
                )?;
                for (i, h_cell) in headers.iter().enumerate() {
                    write!(
                        self.writer,
                        " {}{}",
                        self.color(Style::BOLD),
                        self.color(Style::BRIGHT_CYAN)
                    )?;
                    let rendered = Self::to_plain_string(h_cell);
                    let align = alignments.get(i).copied().unwrap_or(Alignment::Left);
                    let aligned = Self::align_text(&rendered, widths[i], align);
                    write!(self.writer, "{}", aligned)?;
                    write!(
                        self.writer,
                        "{} {}{}{}{}",
                        self.color(Style::RESET),
                        self.color(Style::DIM),
                        self.color(self.style.border_color),
                        v,
                        self.color(Style::RESET)
                    )?;
                }
                writeln!(self.writer)?;

                // Separator row
                self.write_indent()?;
                write!(
                    self.writer,
                    "{}{}{}",
                    self.color(Style::DIM),
                    self.color(self.style.border_color),
                    t_left
                )?;
                for (i, w) in widths.iter().enumerate() {
                    write!(self.writer, "{}", h.repeat(w + 2))?;
                    if i < widths.len() - 1 {
                        write!(self.writer, "{}", cross)?;
                    }
                }
                writeln!(self.writer, "{}{}", t_right, self.color(Style::RESET))?;

                // Body rows
                for row in rows {
                    self.write_indent()?;
                    write!(
                        self.writer,
                        "{}{}{}{}",
                        self.color(Style::DIM),
                        self.color(self.style.border_color),
                        v,
                        self.color(Style::RESET)
                    )?;
                    for (i, w) in widths.iter().enumerate() {
                        write!(self.writer, " ")?;
                        let align = alignments.get(i).copied().unwrap_or(Alignment::Left);
                        if let Some(cell) = row.get(i) {
                            let rendered = Self::to_plain_string(cell);
                            let aligned = Self::align_text(&rendered, *w, align);
                            write!(self.writer, "{}", aligned)?;
                        } else {
                            write!(self.writer, "{:width$}", "", width = *w)?;
                        }
                        write!(
                            self.writer,
                            " {}{}{}{}",
                            self.color(Style::DIM),
                            self.color(self.style.border_color),
                            v,
                            self.color(Style::RESET)
                        )?;
                    }
                    writeln!(self.writer)?;
                }

                // Bottom border
                self.write_indent()?;
                write!(
                    self.writer,
                    "{}{}{}",
                    self.color(Style::DIM),
                    self.color(self.style.border_color),
                    bl
                )?;
                for (i, w) in widths.iter().enumerate() {
                    write!(self.writer, "{}", h.repeat(w + 2))?;
                    if i < widths.len() - 1 {
                        write!(self.writer, "{}", t_up)?;
                    }
                }
                writeln!(self.writer, "{}{}", br, self.color(Style::RESET))?;
                writeln!(self.writer)
            }

            Blockquote(inner) => {
                // Draw left border and indent content
                let border = if self.style.use_unicode_boxes {
                    "▎"
                } else {
                    "|"
                };

                // Render each block individually with proper indentation
                for block in inner.iter() {
                    // Render the block to a string first
                    let mut block_content = String::new();
                    self.indent_level += 1;
                    block.render_with(&mut Renderer::with_style(&mut block_content, self.style))?;
                    self.indent_level -= 1;

                    // Add the border to each line of the block
                    for line in block_content.lines() {
                        self.write_indent()?;
                        write!(
                            self.writer,
                            "{}{}{}{} {}",
                            self.color(Style::DIM),
                            self.color(self.style.border_color),
                            border,
                            self.color(Style::RESET),
                            line
                        )?;
                        writeln!(self.writer)?;
                    }
                }
                writeln!(self.writer)
            }
            Image { alt: _, url: _ } => unimplemented!(),
            HorizontalRule => {
                self.write_indent()?;
                let rule = if self.style.use_unicode_boxes {
                    "─"
                } else {
                    "-"
                };
                writeln!(
                    self.writer,
                    "{}{}{}{}",
                    self.color(Style::DIM),
                    self.color(self.style.border_color),
                    rule.repeat(50),
                    self.color(Style::RESET)
                )?;
                writeln!(self.writer)
            }
            BlockList(inner) => {
                for block in inner.iter() {
                    block.render_with(self)?;
                }
                Ok(())
            }
        }
    }

    /// Render an [`Inline`] as colored terminal output.
    fn render_inline(&mut self, inner: &Inline) -> fmt::Result {
        use Inline::*;

        match inner {
            Text(text) => write!(self.writer, "{}", text),

            Bold(content) => {
                write!(self.writer, "{}", self.color(Style::BOLD))?;
                content.render_with(self)?;
                write!(self.writer, "{}", self.color(Style::RESET))?;
                Ok(())
            }

            Italic(content) => {
                write!(self.writer, "{}", self.color(Style::ITALIC))?;
                content.render_with(self)?;
                write!(self.writer, "{}", self.color(Style::RESET))?;
                Ok(())
            }

            Strikethrough(content) => {
                write!(self.writer, "{}", self.color(Style::STRIKETHROUGH))?;
                content.render_with(self)?;
                write!(self.writer, "{}", self.color(Style::RESET))?;
                Ok(())
            }

            Code(text) => {
                write!(
                    self.writer,
                    "{}{}{}{}{}",
                    self.color(self.style.code_bg),
                    self.color(self.style.code_color),
                    text,
                    self.color(Style::RESET),
                    self.color(Style::RESET)
                )
            }

            Link { text, url } => {
                write!(
                    self.writer,
                    "{}{}",
                    self.color(Style::UNDERLINE),
                    self.color(self.style.link_color)
                )?;
                text.render_with(self)?;
                write!(
                    self.writer,
                    "{} {}{}({}){}",
                    self.color(Style::RESET),
                    self.color(Style::DIM),
                    self.color(self.style.border_color),
                    url,
                    self.color(Style::RESET)
                )?;
                Ok(())
            }
            Image { alt: _, url: _ } => unimplemented!(),
            LineBreak => {
                writeln!(self.writer)?;
                self.write_indent()
            }
        }
    }
}

// Helper methods
impl<'a, W: fmt::Write> Renderer<'a, W> {
    fn align_text(text: &str, width: usize, align: Alignment) -> String {
        let text_len = text.chars().count();
        if text_len >= width {
            text.to_string()
        } else {
            let padding = width - text_len;
            match align {
                Alignment::Left => format!("{:width$}", text, width = width),
                Alignment::Center => {
                    let left_pad = padding / 2;
                    let right_pad = padding - left_pad;
                    format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
                }
                Alignment::Right => format!("{:>width$}", text, width = width),
            }
        }
    }

    fn measure_inline(inline: &Inline) -> usize {
        match inline {
            Inline::Text(t) => t.to_string().chars().count(),
            Inline::Bold(content) | Inline::Italic(content) | Inline::Strikethrough(content) => {
                content.iter().map(Self::measure_inline).sum()
            }
            Inline::Code(t) => t.to_string().chars().count(),
            Inline::Link { text, .. } => text.iter().map(Self::measure_inline).sum(),
            Inline::Image { alt, url } => alt.chars().count() + url.chars().count(),
            Inline::LineBreak => unreachable!(),
        }
    }

    fn to_plain_string(inline: &Inline) -> String {
        match inline {
            Inline::Text(t) => t.to_string(),
            Inline::Bold(content) | Inline::Italic(content) | Inline::Strikethrough(content) => {
                content.iter().map(Self::to_plain_string).collect()
            }
            Inline::Code(t) => t.to_string(),
            Inline::Link { text, .. } => text.iter().map(Self::to_plain_string).collect(),
            Inline::Image { alt: _, url: _ } => unimplemented!(),
            Inline::LineBreak => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::*;

    #[test]
    fn test_terminal_strikethrough() {
        // Simple strikethrough with no colors
        let strike = p(vec![strikethrough("crossed out")]);
        let style = Style::plain();
        let terminal_output = Renderer::to_string_with_style(&strike, style);
        // Should contain the text even without ANSI codes
        assert!(terminal_output.contains("crossed out"));

        // Strikethrough with colors enabled
        let strike = p(vec![
            text("This is "),
            strikethrough("struck"),
            text(" text"),
        ]);
        let terminal_output = Renderer::to_string(&strike);
        // Check for ANSI strikethrough code
        assert!(terminal_output.contains("\x1b[9m"));
        assert!(terminal_output.contains("struck"));

        // Mixed inline styles
        let mixed = p(vec![
            bold("Bold"),
            text(" "),
            italic("italic"),
            text(" "),
            strikethrough("strike"),
        ]);
        let terminal_output = Renderer::to_string(&mixed);
        assert!(terminal_output.contains("\x1b[1m")); // bold
        assert!(terminal_output.contains("\x1b[3m")); // italic
        assert!(terminal_output.contains("\x1b[9m")); // strikethrough
    }

    #[test]
    fn test_terminal_table_alignment() {
        // Create a table with different alignments
        let table = table(
            (
                Align::left("Left"),
                Align::center("Center"),
                Align::right("Right"),
            ),
            vec![
                vec![text("A"), text("B"), text("C")],
                vec![text("Long"), text("Text"), text("Here")],
            ],
        );

        let mut style = Style::ascii();
        style.use_colors = false;
        let output = Renderer::to_string_with_style(&table, style);

        println!("Terminal table with alignments:\n{}", output);

        // The output should have proper spacing
        // Left aligned should have trailing spaces
        // Center aligned should have balanced spaces
        // Right aligned should have leading spaces
        let lines: Vec<&str> = output.lines().collect();

        // Check that the table has the expected structure
        assert!(lines.len() >= 5); // Top border, header, separator, 2 rows, bottom border

        // Verify the content exists (actual alignment is visual)
        assert!(output.contains("Left"));
        assert!(output.contains("Center"));
        assert!(output.contains("Right"));
        assert!(output.contains("Long"));
        assert!(output.contains("Text"));
        assert!(output.contains("Here"));
    }

    #[test]
    fn test_terminal_blockquote() {
        // Simple blockquote test with ASCII style and no colors
        let bq = quote(vec![p("This is a quoted text.")]);
        let mut style = Style::ascii();
        style.use_colors = false; // Disable colors for testing
        let terminal_output = Renderer::to_string_with_style(&bq, style);
        println!("Simple terminal blockquote output:\n{}", terminal_output);
        assert!(terminal_output.contains("| This is a quoted text."));

        // Blockquote with Unicode style
        let bq = quote(vec![p("Fancy quote.")]);
        let terminal_output = Renderer::to_string_with_style(&bq, Style::default());
        println!("Unicode terminal blockquote output:\n{}", terminal_output);
        // Check for the unicode border character and the text (accounting for ANSI codes)
        assert!(terminal_output.contains("▎"));
        assert!(terminal_output.contains("Fancy quote."));

        // Nested blocks in blockquote with ASCII style and no colors
        let bq = quote(vec![h3("Header in Quote"), p("Content in quote.")]);
        let mut style = Style::ascii();
        style.use_colors = false; // Disable colors for testing
        let terminal_output = Renderer::to_string_with_style(&bq, style);
        println!("Complex terminal blockquote output:\n{}", terminal_output);
        assert!(terminal_output.contains("| ### Header in Quote"));
        assert!(terminal_output.contains("| Content in quote."));
    }
}
