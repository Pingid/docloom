use std::fmt;

use super::{Block, Inline, Render, Renderable};

/// Configuration for terminal rendering style
#[derive(Clone)]
pub struct Style {
    pub use_colors: bool,
    pub use_unicode_boxes: bool,
    pub heading_colors: [&'static str; 6],
    pub code_color: &'static str,
    pub code_bg: &'static str,
    pub link_color: &'static str,
    pub list_color: &'static str,
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
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";
    const DIM: &str = "\x1b[2m";
    const ITALIC: &str = "\x1b[3m";
    const UNDERLINE: &str = "\x1b[4m";

    const BRIGHT_BLACK: &str = "\x1b[90m";
    const BRIGHT_CYAN: &str = "\x1b[96m";
    const CYAN: &str = "\x1b[36m";
    const BRIGHT_BLUE: &str = "\x1b[94m";
    const BLUE: &str = "\x1b[34m";
    const BRIGHT_WHITE: &str = "\x1b[97m";
    const GREEN: &str = "\x1b[32m";
    const BRIGHT_GREEN: &str = "\x1b[92m";
    const BRIGHT_YELLOW: &str = "\x1b[93m";
    const BG_BLACK: &str = "\x1b[40m";

    /// Create a style with no colors (plain text)
    pub fn plain() -> Self {
        Self {
            use_colors: false,
            ..Default::default()
        }
    }

    /// Create a style without unicode box-drawing characters
    pub fn ascii() -> Self {
        Self {
            use_unicode_boxes: false,
            ..Default::default()
        }
    }
}

pub struct Renderer<'a, W> {
    writer: &'a mut W,
    indent_level: usize,
    style: Style,
}

impl<'a, W> Renderer<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self::with_style(writer, Style::default())
    }

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
            Paragraph(content) => {
                self.write_indent()?;
                content.render_with(self)?;
                writeln!(self.writer, "{}", self.color(Style::RESET))?;
                writeln!(self.writer)
            }

            Heading { level, content } => {
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
                        "{}{}{}{}{}{}{}",
                        self.color(Style::DIM),
                        self.color(self.style.border_color),
                        left,
                        self.color(Style::RESET),
                        " ",
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
                    if *ordered {
                        write!(
                            self.writer,
                            "{}{}. {}",
                            self.color(self.style.list_color),
                            idx + 1,
                            self.color(Style::RESET)
                        )?;
                    } else {
                        let bullet = if self.style.use_unicode_boxes {
                            "•"
                        } else {
                            "*"
                        };
                        write!(
                            self.writer,
                            "{}{} {}",
                            self.color(self.style.list_color),
                            bullet,
                            self.color(Style::RESET)
                        )?;
                    }

                    self.indent_level += 1;
                    item.render_with(self)?;
                    self.indent_level -= 1;
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
                    write!(self.writer, "{:width$}", rendered, width = widths[i])?;
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
                        if let Some(cell) = row.get(i) {
                            let rendered = Self::to_plain_string(cell);
                            write!(self.writer, "{:width$}", rendered, width = *w)?;
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
        }
    }

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
        }
    }
}

// Helper methods
impl<'a, W: fmt::Write> Renderer<'a, W> {
    fn measure_inline(inline: &Inline) -> usize {
        match inline {
            Inline::Text(t) => t.to_string().chars().count(),
            Inline::Bold(content) | Inline::Italic(content) => {
                content.iter().map(Self::measure_inline).sum()
            }
            Inline::Code(t) => t.to_string().chars().count(),
            Inline::Link { text, .. } => text.iter().map(Self::measure_inline).sum(),
        }
    }

    fn to_plain_string(inline: &Inline) -> String {
        match inline {
            Inline::Text(t) => t.to_string(),
            Inline::Bold(content) | Inline::Italic(content) => {
                content.iter().map(Self::to_plain_string).collect()
            }
            Inline::Code(t) => t.to_string(),
            Inline::Link { text, .. } => text.iter().map(Self::to_plain_string).collect(),
        }
    }
}
