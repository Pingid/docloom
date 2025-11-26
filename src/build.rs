use std::fmt;

use crate::{Alignment, Block, Inline};
use itemize::{IntoItems, IntoRows};

/// Wrap multiple blocks into a [`Block::BlockList`].
pub fn block(value: impl IntoItems<Block>) -> Block {
    Block::BlockList(value.into_items().collect())
}

/// Create a paragraph block from inline elements.
pub fn p(value: impl IntoItems<Inline>) -> Block {
    Block::Paragraph(value.into_items().collect())
}

/// Create a heading block at the provided level.
pub fn h(level: u8, value: impl IntoItems<Inline>) -> Block {
    Block::Heading {
        level,
        content: value.into_items().collect(),
    }
}

/// Create a level-one heading block.
pub fn h1(value: impl IntoItems<Inline>) -> Block {
    h(1, value)
}

/// Create a level-two heading block.
pub fn h2(value: impl IntoItems<Inline>) -> Block {
    h(2, value)
}

/// Create a level-three heading block.
pub fn h3(value: impl IntoItems<Inline>) -> Block {
    h(3, value)
}

/// Create a level-four heading block.
pub fn h4(value: impl IntoItems<Inline>) -> Block {
    h(4, value)
}

/// Create a level-five heading block.
pub fn h5(value: impl IntoItems<Inline>) -> Block {
    h(5, value)
}

/// Create a level-six heading block.
pub fn h6(value: impl IntoItems<Inline>) -> Block {
    h(6, value)
}

/// Create a fenced code block with an optional language hint.
pub fn code_block(language: impl Into<OptString>, value: impl Into<String>) -> Block {
    Block::CodeBlock {
        language: language.into().0,
        content: value.into(),
    }
}

/// Create a list block with an explicit ordering flag.
pub fn list(ordered: bool, items: impl IntoItems<Block>) -> Block {
    Block::List {
        ordered,
        items: items.into_items().collect(),
    }
}

/// Create an unordered list block.
pub fn ul(items: impl IntoItems<Block>) -> Block {
    list(false, items)
}

/// Create an ordered list block.
pub fn ol(items: impl IntoItems<Block>) -> Block {
    list(true, items)
}

#[derive(itemize::IntoItems, itemize::IntoRows)]
#[items_from(types((bool, Block)), tuples(12), collections(vec, slice, array))]
pub struct TaskItem(bool, Block);

impl<T> From<(bool, T)> for TaskItem
where
    T: Into<Block>,
{
    fn from(value: (bool, T)) -> Self {
        TaskItem(value.0, value.1.into())
    }
}

/// Create a task list with checkbox states.
pub fn task_list(items: impl IntoItems<TaskItem>) -> Block {
    Block::TaskList {
        items: items.into_items().map(|item| (item.0, item.1)).collect(),
    }
}

#[derive(itemize::IntoItems)]
#[items_from(types(Inline), tuples(12), collections(vec, slice, array))]
pub struct Align(Alignment, Inline);

impl Align {
    pub fn left(inline: impl Into<Inline>) -> Self {
        Align(Alignment::Left, inline.into())
    }
    pub fn center(inline: impl Into<Inline>) -> Self {
        Align(Alignment::Center, inline.into())
    }
    pub fn right(inline: impl Into<Inline>) -> Self {
        Align(Alignment::Right, inline.into())
    }
}

impl<T> From<T> for Align
where
    T: Into<Inline>,
{
    fn from(value: T) -> Self {
        Align::left(value)
    }
}

/// Create a table with left-aligned columns.
pub fn table(headers: impl IntoItems<Align>, rows: impl IntoRows<Inline>) -> Block {
    let mut columns = Vec::new();
    let mut alignments = Vec::new();
    for header in headers.into_items() {
        columns.push(header.1);
        alignments.push(header.0);
    }
    Block::Table {
        headers: columns,
        rows: rows.into_rows().map(|row| row.collect()).collect(),
        alignments,
    }
}

/// Create a horizontal rule block.
pub fn hr() -> Block {
    Block::HorizontalRule
}

/// Create a blockquote from nested blocks.
pub fn quote(value: impl IntoItems<Block>) -> Block {
    Block::Blockquote(value.into_items().collect())
}

/// Create a text inline node.
pub fn text(value: impl fmt::Display) -> Inline {
    Inline::Text(value.to_string())
}

/// Create a bold inline node.
pub fn bold(value: impl IntoItems<Inline>) -> Inline {
    Inline::Bold(value.into_items().collect())
}

/// Create an italic inline node.
pub fn italic(value: impl IntoItems<Inline>) -> Inline {
    Inline::Italic(value.into_items().collect())
}

/// Create a strikethrough inline node.
pub fn strikethrough(value: impl IntoItems<Inline>) -> Inline {
    Inline::Strikethrough(value.into_items().collect())
}

/// Create an inline code node.
pub fn code(value: impl Into<String>) -> Inline {
    Inline::Code(value.into())
}

/// Create a hyperlink inline node.
pub fn link(text: impl IntoItems<Inline>, url: impl Into<String>) -> Inline {
    Inline::Link {
        text: text.into_items().collect(),
        url: url.into(),
    }
}

/// Extension trait for creating block elements with method syntax.
pub trait BlockExt: Sized + IntoItems<Inline> {
    /// Convert the value into a level-one heading.
    fn h1(self) -> Block {
        h1(self)
    }

    /// Convert the value into a level-two heading.
    fn h2(self) -> Block {
        h2(self)
    }

    /// Convert the value into a level-three heading.
    fn h3(self) -> Block {
        h3(self)
    }

    /// Convert the value into a level-four heading.
    fn h4(self) -> Block {
        h4(self)
    }

    /// Convert the value into a level-five heading.
    fn h5(self) -> Block {
        h5(self)
    }

    /// Convert the value into a level-six heading.
    fn h6(self) -> Block {
        h6(self)
    }

    /// Convert the value into a paragraph block.
    fn p(self) -> Block {
        p(self)
    }
}
impl<T: IntoItems<Inline>> BlockExt for T {}

/// Extension trait for creating inline elements with method syntax.
pub trait InlineExt: Sized {
    /// Wrap the value in bold emphasis.
    fn bold(self) -> Inline
    where
        Self: IntoItems<Inline>,
    {
        bold(self)
    }

    /// Wrap the value in italic emphasis.
    fn italic(self) -> Inline
    where
        Self: IntoItems<Inline>,
    {
        italic(self)
    }

    /// Wrap the value in strikethrough emphasis.
    fn strikethrough(self) -> Inline
    where
        Self: IntoItems<Inline>,
    {
        strikethrough(self)
    }

    /// Create a hyperlink with the value as link text.
    fn link<S>(self, url: S) -> Inline
    where
        Self: IntoItems<Inline>,
        S: Into<String>,
    {
        Inline::Link {
            text: self.into_items().collect(),
            url: url.into(),
        }
    }
}
impl<T: IntoItems<Inline>> InlineExt for T {}

// ---------------- Helper Types ----------------
/// Optional string helper used for code block language parameters.
pub struct OptString(Option<String>);

impl From<&str> for OptString {
    fn from(value: &str) -> Self {
        Self(Some(value.to_string()))
    }
}
impl From<String> for OptString {
    fn from(value: String) -> Self {
        Self(Some(value))
    }
}
impl From<()> for OptString {
    fn from(_: ()) -> Self {
        Self(None)
    }
}

impl From<Option<String>> for OptString {
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}
