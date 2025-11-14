use std::fmt;

use super::{Block, Inline, Render, Renderable};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Style {
    pub code_fence: FenceStyle,
    pub list_marker: ListMarker,
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum FenceStyle {
    Backtick,
    Tilde,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ListMarker {
    Asterisk,
    Dash,
}

pub struct Renderer<'a, W> {
    writer: &'a mut W,
    style: Style,
}

impl<'a, W> Renderer<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self::with_style(writer, Style::default())
    }

    pub fn with_style(writer: &'a mut W, style: Style) -> Self {
        Self { writer, style }
    }
}

impl Renderer<'_, String> {
    pub fn to_string<R>(r: &R) -> String
    where
        R: for<'b> Renderable<Renderer<'b, String>, Output = Result<(), fmt::Error>> + ?Sized,
    {
        Self::to_string_with_style(r, Style::default())
    }

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

                // separator row
                write!(self.writer, "|")?;
                for w in &widths {
                    let dashes = "-".repeat(*w);
                    write!(self.writer, " {dashes} |")?;
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
            HorizontalRule => writeln!(self.writer, "---"),
        }
    }

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
            Code(text) => write!(self.writer, "`{text}`"),
            Link { text, url } => {
                write!(self.writer, "[")?;
                text.render_with(self)?;
                write!(self.writer, "]({url})")?;
                Ok(())
            }
        }
    }
}

impl<'a, W: fmt::Write> Renderer<'a, W> {
    fn measure_inline(inline: &Inline) -> usize {
        match inline {
            Inline::Text(t) => t.to_string().chars().count(),
            Inline::Bold(content) | Inline::Italic(content) => {
                content.iter().map(Self::measure_inline).sum::<usize>() + 2
            }
            Inline::Code(t) => t.to_string().chars().count() + 2,
            Inline::Link { text, .. } => 2 + text.iter().map(Self::measure_inline).sum::<usize>(),
        }
    }
}
