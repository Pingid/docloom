use std::fmt;

use crate::into_vec::{ToRows, ToVec};
use crate::{Alignment, Block, Inline};

pub fn block(value: impl ToVec<Block>) -> Block {
    Block::BlockList(value.to_vec())
}

pub fn p(value: impl ToVec<Inline>) -> Block {
    Block::Paragraph(value.to_vec())
}

pub fn h(level: u8, value: impl ToVec<Inline>) -> Block {
    Block::Heading {
        level,
        content: value.to_vec(),
    }
}

pub fn h1(value: impl ToVec<Inline>) -> Block {
    h(1, value)
}

pub fn h2(value: impl ToVec<Inline>) -> Block {
    h(2, value)
}

pub fn h3(value: impl ToVec<Inline>) -> Block {
    h(3, value)
}

pub fn h4(value: impl ToVec<Inline>) -> Block {
    h(4, value)
}

pub fn h5(value: impl ToVec<Inline>) -> Block {
    h(5, value)
}

pub fn h6(value: impl ToVec<Inline>) -> Block {
    h(6, value)
}

pub fn code_block(language: Option<String>, value: impl Into<String>) -> Block {
    Block::CodeBlock {
        language: language.into(),
        content: value.into(),
    }
}

pub fn list(ordered: bool, items: impl ToVec<Block>) -> Block {
    Block::List {
        ordered,
        items: items.to_vec().into(),
    }
}

pub fn ul(items: impl ToVec<Block>) -> Block {
    list(false, items)
}

pub fn ol(items: impl ToVec<Block>) -> Block {
    list(true, items)
}

pub fn task_list(items: impl ToVec<(bool, Block)>) -> Block {
    Block::TaskList {
        items: items
            .to_vec()
            .into_iter()
            .map(|(checked, item)| (checked, item))
            .collect(),
    }
}

pub fn table(headers: impl ToVec<Inline>, rows: impl ToRows<Inline>) -> Block {
    let headers: Vec<Inline> = headers.to_vec();
    let alignments = vec![Alignment::Left; headers.len()];
    Block::Table {
        headers,
        rows: rows.to_rows(),
        alignments,
    }
}

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

pub fn hr() -> Block {
    Block::HorizontalRule
}

pub fn quote(value: impl ToVec<Block>) -> Block {
    Block::Blockquote(value.to_vec())
}

pub fn text(value: impl fmt::Display) -> Inline {
    Inline::Text(value.to_string())
}

pub fn bold(value: impl ToVec<Inline>) -> Inline {
    Inline::Bold(value.to_vec())
}

pub fn italic(value: impl ToVec<Inline>) -> Inline {
    Inline::Italic(value.to_vec())
}

pub fn strikethrough(value: impl ToVec<Inline>) -> Inline {
    Inline::Strikethrough(value.to_vec())
}

pub fn code(value: impl Into<String>) -> Inline {
    Inline::Code(value.into())
}

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

/// Extension trait for creating block elements with method syntax
pub trait BlockExt: Sized + ToVec<Inline> {
    fn h1(self) -> Block {
        h1(self)
    }

    fn h2(self) -> Block {
        h2(self)
    }

    fn h3(self) -> Block {
        h3(self)
    }

    fn h4(self) -> Block {
        h4(self)
    }

    fn h5(self) -> Block {
        h5(self)
    }

    fn h6(self) -> Block {
        h6(self)
    }

    fn p(self) -> Block {
        p(self)
    }
}
impl<T: ToVec<Inline>> BlockExt for T {}

/// Extension trait for creating inline elements with method syntax
pub trait InlineExt: Sized + ToVec<Inline> {
    fn bold(self) -> Inline {
        bold(self)
    }
    fn italic(self) -> Inline {
        italic(self)
    }
    fn strikethrough(self) -> Inline {
        strikethrough(self)
    }
    fn link(self, url: impl Into<String>) -> Inline {
        link(self, url)
    }
}
impl<T: ToVec<Inline>> InlineExt for T {}
