//! Markdown renderer and supporting types.
//!
//! The `md` module turns [`crate::Block`] and [`crate::Inline`]
//! structures into Markdown text with customizable styling.
//!
//! # Examples
//! ```rust
//! use docloom::md::{FenceStyle, ListMarker, Style, doc};
//! use docloom::prelude::*;
//!
//! // Optional style configuration
//! let style = Style {
//!     code_fence: FenceStyle::Tilde,
//!     list_marker: ListMarker::Asterisk,
//!     max_heading: 3,
//! };
//!
//! let rendered = doc([
//!     h1("Docloom"),
//!     p("Render Markdown from structured blocks."),
//!     ul(["Configure Markdown output.", "Render it anywhere."]),
//! ])
//! .with_style(style)
//! .to_string();
//! ```

use std::fmt;

use super::{Block, Inline, Render, Renderable};
use crate::{Alignment, into_vec::ToVec};

/// Markdown document wrapper that renders blocks with a [`Style`].
pub struct Doc {
    content: Vec<Block>,
    style: Style,
}

impl Doc {
    /// Create a new document from items convertible to [`Block`].
    pub fn new(value: impl ToVec<Block>) -> Self {
        Self {
            content: value.to_vec(),
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
pub fn doc(value: impl ToVec<Block>) -> Doc {
    Doc::new(value)
}

/// Configuration values that affect Markdown output.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Style {
    /// Fence style to use when rendering code blocks.
    pub code_fence: FenceStyle,
    /// Bullet marker to use for unordered lists.
    pub list_marker: ListMarker,
    /// Maximum heading level emitted when rendering blocks.
    pub max_heading: u8,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            code_fence: FenceStyle::Backtick,
            list_marker: ListMarker::Dash,
            max_heading: 6,
        }
    }
}

/// Fence marker options for code blocks.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum FenceStyle {
    /// Render code blocks using backtick fences.
    Backtick,
    /// Render code blocks using tilde fences.
    Tilde,
}

/// Marker options for unordered lists.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ListMarker {
    /// Use `*` for unordered list items.
    Asterisk,
    /// Use `-` for unordered list items.
    Dash,
}

/// Renderer that writes Markdown to any [`fmt::Write`] target.
pub struct Renderer<'a, W> {
    writer: &'a mut W,
    style: Style,
}

impl<'a, W> Renderer<'a, W> {
    /// Create a renderer that writes to `writer` with the default [`Style`].
    pub fn new(writer: &'a mut W) -> Self {
        Self::with_style(writer, Style::default())
    }

    /// Create a renderer with a custom [`Style`].
    pub fn with_style(writer: &'a mut W, style: Style) -> Self {
        Self { writer, style }
    }

    /// Render an arbitrary [`crate::Renderable`] value to the writer.
    pub fn render<R>(&mut self, r: &R) -> Result<(), fmt::Error>
    where
        R: for<'b> Renderable<Renderer<'b, W>, Output = Result<(), fmt::Error>> + ?Sized,
    {
        r.render_with(self)
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

    /// Render a value to a [`String`] using the default style, returning errors.
    pub fn try_to_string<R>(r: &R) -> Result<String, fmt::Error>
    where
        R: for<'b> Renderable<Renderer<'b, String>, Output = Result<(), fmt::Error>> + ?Sized,
    {
        Self::try_to_string_with_style(r, Style::default())
    }

    /// Render a value to a [`String`] with a custom [`Style`].
    pub fn to_string_with_style<R>(r: &R, style: Style) -> String
    where
        R: for<'b> Renderable<Renderer<'b, String>, Output = Result<(), fmt::Error>> + ?Sized,
    {
        Self::try_to_string_with_style(r, style).unwrap()
    }

    /// Render a value to a [`String`] with a custom [`Style`], returning errors.
    pub fn try_to_string_with_style<R>(r: &R, style: Style) -> Result<String, fmt::Error>
    where
        R: for<'b> Renderable<Renderer<'b, String>, Output = Result<(), fmt::Error>> + ?Sized,
    {
        let mut buf = String::new();
        r.render_with(&mut Renderer::with_style(&mut buf, style))?;
        Ok(buf)
    }
}

impl<'a, W: fmt::Write> Render for Renderer<'a, W> {
    type Output = Result<(), fmt::Error>;

    /// Render a [`crate::Block`] into Markdown.
    fn render_block(&mut self, inner: &Block) -> Self::Output {
        use Block::*;
        match inner {
            Paragraph(inner) => {
                inner.render_with(self)?;
                writeln!(self.writer)
            }
            Heading { level, content } => {
                // Apply max_heading style
                let clamped_level = (*level).min(self.style.max_heading);
                write!(self.writer, "{} ", "#".repeat(clamped_level as usize))?;
                content.render_with(self)?;
                writeln!(self.writer)
            }
            CodeBlock { language, content } => {
                // Apply code_fence style
                let fence = match self.style.code_fence {
                    FenceStyle::Backtick => "```",
                    FenceStyle::Tilde => "~~~",
                };

                if let Some(lang) = language {
                    writeln!(self.writer, "{}{}", fence, lang)?;
                } else {
                    writeln!(self.writer, "{}", fence)?;
                }
                writeln!(self.writer, "{content}")?;
                writeln!(self.writer, "{}", fence)?;
                writeln!(self.writer)
            }
            List { ordered, items } => {
                for (idx, item) in items.iter().enumerate() {
                    if *ordered {
                        write!(self.writer, "{}. ", idx + 1)?;
                    } else {
                        // Apply list_marker style
                        let marker = match self.style.list_marker {
                            ListMarker::Asterisk => "*",
                            ListMarker::Dash => "-",
                        };
                        write!(self.writer, "{} ", marker)?;
                    }
                    item.render_with(self)?;
                    writeln!(self.writer)?;
                }
                writeln!(self.writer)
            }
            TaskList { items } => {
                for (checked, item) in items.iter() {
                    let mark = if *checked { "x" } else { " " };
                    write!(self.writer, "- [{mark}] ")?;
                    item.render_with(self)?;
                    writeln!(self.writer)?;
                }
                writeln!(self.writer)
            }
            Table {
                headers,
                rows,
                alignments,
            } => {
                let col_count = headers.len();
                let mut widths: Vec<usize> =
                    headers.iter().map(|h| Self::measure_inline(h)).collect();

                for row in rows {
                    for (i, cell) in row.iter().enumerate().take(col_count) {
                        widths[i] = widths[i].max(Self::measure_inline(cell));
                    }
                }

                // at least 3 chars for separator row
                for w in &mut widths {
                    *w = (*w).max(3);
                }

                // header row
                write!(self.writer, "|")?;
                for (i, h) in headers.iter().enumerate() {
                    // Render to a temporary string first
                    let rendered = Renderer::to_string(h);
                    write!(self.writer, " {:width$} |", rendered, width = widths[i])?;
                }
                writeln!(self.writer)?;

                // separator row with alignment
                write!(self.writer, "|")?;
                for (w, align) in widths.iter().zip(alignments.iter()) {
                    let spec = match align {
                        Alignment::Left => {
                            let dashes = "-".repeat((*w).saturating_sub(1));
                            format!(":{dashes}")
                        }
                        Alignment::Center => {
                            let dashes = "-".repeat((*w).saturating_sub(2).max(1));
                            format!(":{dashes}:")
                        }
                        Alignment::Right => {
                            let dashes = "-".repeat((*w).saturating_sub(1));
                            format!("{dashes}:")
                        }
                    };
                    write!(self.writer, " {:width$} |", spec, width = *w)?;
                }
                writeln!(self.writer)?;

                // body rows
                for row in rows {
                    write!(self.writer, "|")?;
                    for (i, w) in widths.iter().enumerate() {
                        if let Some(cell) = row.get(i) {
                            let rendered = Renderer::to_string(cell);
                            write!(self.writer, " {:width$} |", rendered, width = *w)?;
                        } else {
                            // Empty cell if row doesn't have enough columns
                            write!(self.writer, " {:width$} |", "", width = *w)?;
                        }
                    }
                    writeln!(self.writer)?;
                }

                writeln!(self.writer)
            }
            Blockquote(inner) => {
                // Render each block individually and add to blockquote
                for (i, block) in inner.iter().enumerate() {
                    // Render this block to a string
                    let mut block_content = String::new();
                    block.render_with(&mut Renderer::with_style(&mut block_content, self.style))?;

                    // Remove trailing newlines from the block content
                    let block_content = block_content.trim_end();

                    // Add "> " prefix to each line of this block
                    for line in block_content.lines() {
                        writeln!(self.writer, "> {}", line)?;
                    }

                    // Add a blank blockquote line between blocks (except after the last one)
                    if i < inner.len() - 1 {
                        writeln!(self.writer, ">")?;
                    }
                }
                Ok(())
            }
            Image { alt, url } => writeln!(self.writer, "![{alt}]({url})"),
            HorizontalRule => writeln!(self.writer, "---"),
            BlockList(inner) => {
                for block in inner.iter() {
                    block.render_with(self)?;
                }
                Ok(())
            }
        }
    }

    /// Render an [`crate::Inline`] into Markdown.
    fn render_inline(&mut self, inner: &Inline) -> fmt::Result {
        use Inline::*;
        match inner {
            Text(text) => write!(self.writer, "{text}"),
            Bold(inner) => {
                write!(self.writer, "**")?;
                inner.render_with(self)?;
                write!(self.writer, "**")?;
                Ok(())
            }
            Italic(inner) => {
                write!(self.writer, "*")?;
                inner.render_with(self)?;
                write!(self.writer, "*")?;
                Ok(())
            }
            Strikethrough(inner) => {
                write!(self.writer, "~~")?;
                inner.render_with(self)?;
                write!(self.writer, "~~")?;
                Ok(())
            }
            Code(text) => write!(self.writer, "`{text}`"),
            Link { text, url } => {
                write!(self.writer, "[")?;
                text.render_with(self)?;
                write!(self.writer, "]({url})")?;
                Ok(())
            }
            Image { alt, url } => writeln!(self.writer, "![{alt}]({url})"),
            LineBreak => write!(self.writer, "  \n"),
        }
    }
}

impl<'a, W: fmt::Write> Renderer<'a, W> {
    fn measure_inline(inline: &Inline) -> usize {
        match inline {
            Inline::Text(t) => t.to_string().chars().count(),
            Inline::Bold(content) | Inline::Italic(content) => {
                content.iter().map(Self::measure_inline).sum::<usize>() + 4
            }
            Inline::Strikethrough(content) => {
                content.iter().map(Self::measure_inline).sum::<usize>() + 4
            }
            Inline::Code(t) => t.to_string().chars().count() + 2,
            Inline::Link { text, .. } => 2 + text.iter().map(Self::measure_inline).sum::<usize>(),
            Inline::Image { alt, url } => {
                5 + alt.to_string().chars().count() + url.to_string().chars().count()
            }
            Inline::LineBreak => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::*;

    #[test]
    fn test_markdown_table() {
        let table = table(("Name", "Age"), (("Alice", "30"), ("Bob", "25")));
        let markdown = Renderer::to_string(&table);
        println!("{markdown}");
        assert_eq!(
            markdown.trim(),
            r#"
| Name  | Age |
| :---- | :-- |
| Alice | 30  |
| Bob   | 25  |
            "#
            .trim()
        );
    }

    #[test]
    fn test_markdown_table_with_alignment() {
        use crate::Alignment;

        let table = table_aligned(
            ("Name", "Age", "Score"),
            (("Alice", "30", "95"), ("Bob", "25", "87")),
            vec![Alignment::Left, Alignment::Center, Alignment::Right],
        );
        let markdown = Renderer::to_string(&table);
        println!("{markdown}");
        // Check the separator row contains proper alignment specs
        let lines: Vec<&str> = markdown.lines().collect();
        let separator = lines[1]; // Second line is the separator
        assert!(separator.contains(":----")); // left alignment
        assert!(separator.contains(":-:")); // center alignment (Age has 3 chars)
        assert!(separator.contains("----:")); // right alignment
    }

    #[test]
    fn test_markdown_strikethrough() {
        // Simple strikethrough
        let strike = p(vec![strikethrough("crossed out text")]);
        let markdown = Renderer::to_string(&strike);
        assert_eq!(markdown.trim(), "~~crossed out text~~");

        // Strikethrough with nested content
        let strike = p((
            "This has ",
            strikethrough(("struck ", "bold".bold(), " text")),
            " in it.",
        ));
        let markdown = Renderer::to_string(&strike);
        assert_eq!(markdown.trim(), "This has ~~struck **bold** text~~ in it.");

        // Multiple inline styles
        let mixed = p((
            bold("Bold"),
            text(", "),
            "italic".italic(),
            ", ",
            "strikethrough".strikethrough(),
            ", and ",
            code("code"),
        ));
        let markdown = Renderer::to_string(&mixed);
        assert_eq!(
            markdown.trim(),
            "**Bold**, *italic*, ~~strikethrough~~, and `code`"
        );
    }

    #[test]
    fn test_markdown_blockquote() {
        // Simple blockquote with a paragraph
        let bq = quote(vec![p("This is a quoted paragraph.")]);
        let markdown = Renderer::to_string(&bq);
        println!("Simple blockquote output:\n{}", markdown);
        assert_eq!(markdown.trim(), "> This is a quoted paragraph.");

        // Blockquote with multiple paragraphs
        let bq = quote(vec![p("First paragraph."), p("Second paragraph.")]);
        let markdown = Renderer::to_string(&bq);
        println!("Multiple paragraphs blockquote output:\n{}", markdown);
        println!("Trimmed output:\n{}", markdown.trim());
        // The markdown renderer should preserve blank lines between paragraphs
        assert_eq!(
            markdown.trim(),
            "> First paragraph.\n>\n> Second paragraph."
        );

        // Blockquote with different block types
        let bq = quote(vec![
            h2("Quote Header"),
            p("Quote content."),
            list(false, vec![p("Item 1"), p("Item 2")]),
        ]);
        let markdown = Renderer::to_string(&bq);
        println!("Complex blockquote output:\n{}", markdown);
        assert!(markdown.contains("> ## Quote Header"));
        assert!(markdown.contains("> Quote content."));
        assert!(markdown.contains("> - Item 1"));
        assert!(markdown.contains("> - Item 2"));
    }
}
