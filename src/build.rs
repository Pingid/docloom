use std::fmt;

use crate::into_vec::{ToRows, ToVec};
use crate::{Alignment, Block, Inline};

/// Wrap multiple blocks into a [`Block::BlockList`].
pub fn block(value: impl ToVec<Block>) -> Block {
    Block::BlockList(value.to_vec())
}

/// Create a paragraph block from inline elements.
pub fn p(value: impl ToVec<Inline>) -> Block {
    Block::Paragraph(value.to_vec())
}

/// Create a heading block at the provided level.
pub fn h(level: u8, value: impl ToVec<Inline>) -> Block {
    Block::Heading {
        level,
        content: value.to_vec(),
    }
}

/// Create a level-one heading block.
pub fn h1(value: impl ToVec<Inline>) -> Block {
    h(1, value)
}

/// Create a level-two heading block.
pub fn h2(value: impl ToVec<Inline>) -> Block {
    h(2, value)
}

/// Create a level-three heading block.
pub fn h3(value: impl ToVec<Inline>) -> Block {
    h(3, value)
}

/// Create a level-four heading block.
pub fn h4(value: impl ToVec<Inline>) -> Block {
    h(4, value)
}

/// Create a level-five heading block.
pub fn h5(value: impl ToVec<Inline>) -> Block {
    h(5, value)
}

/// Create a level-six heading block.
pub fn h6(value: impl ToVec<Inline>) -> Block {
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
pub fn list(ordered: bool, items: impl ToVec<Block>) -> Block {
    Block::List {
        ordered,
        items: items.to_vec().into(),
    }
}

/// Create an unordered list block.
pub fn ul(items: impl ToVec<Block>) -> Block {
    list(false, items)
}

/// Create an ordered list block.
pub fn ol(items: impl ToVec<Block>) -> Block {
    list(true, items)
}

/// Create a task list with checkbox states.
pub fn task_list(items: impl ToVec<(bool, Block)>) -> Block {
    Block::TaskList {
        items: items
            .to_vec()
            .into_iter()
            .map(|(checked, item)| (checked, item))
            .collect(),
    }
}

/// Create a table with left-aligned columns.
pub fn table(headers: impl ToVec<Inline>, rows: impl ToRows<Inline>) -> Block {
    let headers: Vec<Inline> = headers.to_vec();
    let alignments = vec![Alignment::Left; headers.len()];
    Block::Table {
        headers,
        rows: rows.to_rows(),
        alignments,
    }
}

/// Create a table with explicit alignments.
pub fn table_aligned(
    headers: impl ToVec<Inline>,
    rows: impl ToRows<Inline>,
    alignments: impl Into<Vec<Alignment>>,
) -> Block {
    Block::Table {
        headers: headers.to_vec(),
        rows: rows.to_rows(),
        alignments: alignments.into().into(),
    }
}

/// Create a horizontal rule block.
pub fn hr() -> Block {
    Block::HorizontalRule
}

/// Create a blockquote from nested blocks.
pub fn quote(value: impl ToVec<Block>) -> Block {
    Block::Blockquote(value.to_vec())
}

/// Create a text inline node.
pub fn text(value: impl fmt::Display) -> Inline {
    Inline::Text(value.to_string())
}

/// Create a bold inline node.
pub fn bold(value: impl ToVec<Inline>) -> Inline {
    Inline::Bold(value.to_vec())
}

/// Create an italic inline node.
pub fn italic(value: impl ToVec<Inline>) -> Inline {
    Inline::Italic(value.to_vec())
}

/// Create a strikethrough inline node.
pub fn strikethrough(value: impl ToVec<Inline>) -> Inline {
    Inline::Strikethrough(value.to_vec())
}

/// Create an inline code node.
pub fn code(value: impl Into<String>) -> Inline {
    Inline::Code(value.into())
}

/// Create a hyperlink inline node.
pub fn link(text: impl ToVec<Inline>, url: impl Into<String>) -> Inline {
    Inline::Link {
        text: text.to_vec(),
        url: url.into(),
    }
}

// ---------------- Block Trait impls ----------------
impl<T> From<T> for Block
where
    T: Into<Inline>,
{
    fn from(value: T) -> Self {
        Block::Paragraph(vec![value.into()])
    }
}
crate::impl_to_vec!(Block, Block, 'a Block, Inline, 'a Inline, 'a str, String, 'a String, usize, bool, f32, f64);

// ---------------- Inline Trait impls ----------------
impl<T> From<T> for Inline
where
    T: std::fmt::Display,
{
    fn from(value: T) -> Self {
        Inline::Text(value.to_string())
    }
}
crate::impl_to_vec!(Inline, Inline, 'a Inline, 'a str, String, 'a String, usize, bool, f32, f64);

// ---------------- Extension Traits ----------------

/// Extension trait for creating block elements with method syntax.
pub trait BlockExt: Sized + ToVec<Inline> {
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
impl<T: ToVec<Inline>> BlockExt for T {}

/// Extension trait for creating inline elements with method syntax.
pub trait InlineExt: Sized {
    /// Wrap the value in bold emphasis.
    fn bold(self) -> Inline
    where
        Self: ToVec<Inline>,
    {
        bold(self)
    }

    /// Wrap the value in italic emphasis.
    fn italic(self) -> Inline
    where
        Self: ToVec<Inline>,
    {
        italic(self)
    }

    /// Wrap the value in strikethrough emphasis.
    fn strikethrough(self) -> Inline
    where
        Self: ToVec<Inline>,
    {
        strikethrough(self)
    }

    /// Create a hyperlink with the value as link text.
    fn link<S>(self, url: S) -> Inline
    where
        Self: ToVec<Inline>,
        S: Into<String>,
    {
        Inline::Link {
            text: ToVec::to_vec(self),
            url: url.into(),
        }
    }
}
impl<T: ToVec<Inline>> InlineExt for T {}

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
