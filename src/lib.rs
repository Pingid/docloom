pub mod build;
mod macros;
pub mod markdown;
pub mod terminal;

pub use build::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Block {
    Paragraph(Vec<Inline>),
    Heading {
        level: u8,
        content: Vec<Inline>,
    },
    CodeBlock {
        language: Option<String>,
        content: String,
    },
    List {
        ordered: bool,
        items: Vec<Block>,
    },
    Table {
        headers: Vec<Inline>,
        rows: Vec<Vec<Inline>>,
        alignments: Vec<Alignment>,
    },
    HorizontalRule,
}

impl<T> From<T> for Block
where
    T: Into<Inline>,
{
    fn from(value: T) -> Self {
        Block::Paragraph(vec![value.into()]).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Inline {
    Text(String),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    Code(String),
    Link { text: Vec<Inline>, url: String },
}

impl<T> From<T> for Inline
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Inline::Text(value.into())
    }
}

pub trait Render {
    type Output;
    fn render_block(&mut self, inner: &Block) -> Self::Output;
    fn render_inline(&mut self, inner: &Inline) -> Self::Output;
}

pub trait Renderable<R> {
    type Output;
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
    fn render_with(&self, renderer: &mut R) -> Result<(), E> {
        for item in self {
            item.render_with(renderer)?;
        }
        Ok(())
    }
}
