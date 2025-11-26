//! Flexible tools for assembling and rendering structured documentation trees.
//!
//! Build documents from [`Block`] and [`Inline`] nodes, then render them using
//! Markdown-friendly [`md`] output, ANSI-aware [`term`] output, or a custom
//! [`Render`] implementation.
//!
//! # Examples
//! ```rust
//! use docloom::md::doc;
//! use docloom::prelude::*;
//!
//! let markdown = doc([
//!     h1("Docloom Overview"),
//!     p((bold("Docloom"), " turns structured blocks into Markdown.")),
//!     h2("Getting Started"),
//!     p((
//!         "Compose ",
//!         italic("inline styles"),
//!         " and render them together.",
//!     )),
//!     code_block("rust", "fn main() { println!(\"hello\"); }"),
//!     h2("Lists"),
//!     ul(["Supports bullet lists", "And numbered ones"]),
//!     ol(["Call `doc`", "Render the output"]),
//!     task_list([
//!         (true, p("Choose block types")),
//!         (false, p("Render to more targets")),
//!     ]),
//!     h2("Tables"),
//!     table(
//!         ("Feature", "Description"),
//!         (
//!             ("Tables", "Markdown alignment helpers"),
//!             ("Task lists", "Checkbox formatting"),
//!         ),
//!     ),
//!     h2("Quotes"),
//!     quote(p("Render nested content with ease.")),
//!     hr(),
//!     p("Generate complete documents without manual Markdown stitching."),
//! ]);
//! let content = format!("{markdown}");
//! assert!(content.contains("Docloom Overview"));
//! ```

mod build;

pub mod md;
pub mod term;

/// Convenience re-exports of builder helpers and extension traits.
pub mod prelude {
    pub use crate::build::{
        Align, BlockExt, InlineExt, block, bold, code, code_block, h1, h2, h3, h4, h5, h6, hr,
        italic, link, ol, p, quote, strikethrough, table, task_list, text, ul,
    };
}

/// Document-level nodes describing paragraphs, lists, and other structural items.
#[derive(Debug, Clone, PartialEq, Eq, Hash, itemize::IntoItems, itemize::IntoRows)]
#[items_from(types(Block, Inline), tuples(12), collections(vec, slice, array))]
pub enum Block {
    /// A paragraph of inline elements.
    Paragraph(Vec<Inline>),
    /// A heading with a specific level and inline content.
    Heading { level: u8, content: Vec<Inline> },
    /// A fenced code block with an optional language tag.
    CodeBlock {
        language: Option<String>,
        content: String,
    },
    /// A nested collection of quoted blocks.
    Blockquote(Vec<Block>),
    /// An ordered or unordered list of blocks.
    List { ordered: bool, items: Vec<Block> },
    /// A list of checkbox items paired with their content.
    TaskList { items: Vec<(bool, Block)> },
    /// A table with headers, rows, and column alignments.
    Table {
        headers: Vec<Inline>,
        rows: Vec<Vec<Inline>>,
        alignments: Vec<Alignment>,
    },
    /// A standalone image block.
    Image { alt: String, url: String },
    /// A thematic break separating sections.
    HorizontalRule,
    /// A container that renders nested blocks in sequence.
    BlockList(Vec<Block>),
}

impl<T> From<T> for Block
where
    T: Into<Inline>,
{
    fn from(value: T) -> Self {
        Block::Paragraph(vec![value.into()])
    }
}

/// Inline elements that compose textual content.
#[derive(Debug, Clone, PartialEq, Eq, Hash, itemize::IntoItems, itemize::IntoRows)]
#[items_from(types(&'a str, String, &'a String, usize, bool, f32, f64), tuples(12), collections(vec, slice, array))]
pub enum Inline {
    /// Plain text.
    Text(String),
    /// Bold inline content.
    Bold(Vec<Inline>),
    /// Italic inline content.
    Italic(Vec<Inline>),
    /// Struck-through inline content.
    Strikethrough(Vec<Inline>),
    /// Inline code snippet.
    Code(String),
    /// A hyperlink with inline text and destination.
    Link { text: Vec<Inline>, url: String },
    /// An inline image reference with alternate text.
    Image { alt: String, url: String },
    /// A hard line break.
    LineBreak,
}

impl<T> From<T> for Inline
where
    T: std::fmt::Display,
{
    fn from(value: T) -> Self {
        Inline::Text(value.to_string())
    }
}

/// Column alignment options used when rendering tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    /// Align content with the leading edge.
    Left,
    /// Center content within the column.
    Center,
    /// Align content with the trailing edge.
    Right,
}

/// Trait implemented by renderers that consume [`Block`] and [`Inline`] trees.
pub trait Render {
    type Output;
    /// Render a single document block.
    fn render_block(&mut self, inner: &Block) -> Self::Output;
    /// Render a single inline node.
    fn render_inline(&mut self, inner: &Inline) -> Self::Output;
}

/// Trait for types that know how to render themselves with a [`Render`].
pub trait Renderable<R> {
    type Output;
    /// Render `self` with the provided renderer, producing an output value.
    fn render_with(&self, renderer: &mut R) -> Self::Output;
}

impl<R: Render> Renderable<R> for Block {
    type Output = R::Output;
    fn render_with(&self, renderer: &mut R) -> Self::Output {
        renderer.render_block(self)
    }
}

impl<R: Render> Renderable<R> for Inline {
    type Output = R::Output;
    fn render_with(&self, renderer: &mut R) -> Self::Output {
        renderer.render_inline(self)
    }
}

impl<R, U, E> Renderable<R> for [U]
where
    U: Renderable<R, Output = Result<(), E>>,
{
    type Output = U::Output;
    /// Render every element in a slice sequentially, propagating errors.
    fn render_with(&self, renderer: &mut R) -> Result<(), E> {
        for item in self {
            item.render_with(renderer)?;
        }
        Ok(())
    }
}
