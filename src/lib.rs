mod build;
pub(crate) mod into_vec;

pub mod markdown;
pub mod terminal;

pub mod prelude {
    pub use crate::build::*;
}

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
    Blockquote(Vec<Block>),
    List {
        ordered: bool,
        items: Vec<Block>,
    },
    TaskList {
        items: Vec<(bool, Block)>,
    },
    Table {
        headers: Vec<Inline>,
        rows: Vec<Vec<Inline>>,
        alignments: Vec<Alignment>,
    },
    Image {
        alt: String,
        url: String,
    },
    HorizontalRule,
    BlockList(Vec<Block>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Inline {
    Text(String),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    Strikethrough(Vec<Inline>),
    Code(String),
    Link { text: Vec<Inline>, url: String },
    Image { alt: String, url: String },
    LineBreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    Left,
    Center,
    Right,
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
