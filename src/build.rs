pub struct DocBuild {
    blocks: Vec<crate::Block>,
}

impl DocBuild {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn h1(mut self, text: impl Into<crate::Inline>) -> Self {
        self.blocks.push(crate::Block::Heading {
            level: 1,
            content: vec![text.into()],
        });
        self
    }

    pub fn h2(mut self, text: impl Into<crate::Inline>) -> Self {
        self.blocks.push(crate::Block::Heading {
            level: 2,
            content: vec![text.into()],
        });
        self
    }

    pub fn p(mut self, f: impl FnOnce(InlineBuild) -> InlineBuild) -> Self {
        let builder = InlineBuild::new();
        let content = f(builder).build();
        self.blocks.push(crate::Block::Paragraph(content));
        self
    }

    pub fn code(mut self, lang: impl Into<String>, content: impl Into<String>) -> Self {
        self.blocks.push(crate::Block::CodeBlock {
            language: Some(lang.into()),
            content: content.into(),
        });
        self
    }

    pub fn ul(mut self, f: impl FnOnce(ListBuild) -> ListBuild) -> Self {
        let builder = ListBuild::new();
        let items = f(builder).build();
        self.blocks.push(crate::Block::List {
            ordered: false,
            items,
        });
        self
    }

    pub fn hr(mut self) -> Self {
        self.blocks.push(crate::Block::HorizontalRule);
        self
    }

    pub fn build(self) -> Vec<crate::Block> {
        self.blocks
    }
}

pub struct InlineBuild {
    inlines: Vec<crate::Inline>,
}

impl InlineBuild {
    pub fn new() -> Self {
        Self { inlines: vec![] }
    }

    pub fn text(mut self, text: impl Into<crate::Inline>) -> Self {
        self.inlines.push(text.into());
        self
    }

    pub fn bold(mut self, text: impl Into<crate::Inline>) -> Self {
        self.inlines.push(crate::Inline::Bold(vec![text.into()]));
        self
    }

    pub fn italic(mut self, text: impl Into<crate::Inline>) -> Self {
        self.inlines.push(crate::Inline::Italic(vec![text.into()]));
        self
    }

    pub fn code(mut self, text: impl Into<String>) -> Self {
        self.inlines.push(crate::Inline::Code(text.into()));
        self
    }

    pub fn link(mut self, text: impl Into<crate::Inline>, url: impl Into<String>) -> Self {
        self.inlines.push(crate::Inline::Link {
            text: vec![text.into()],
            url: url.into(),
        });
        self
    }

    pub fn build(self) -> Vec<crate::Inline> {
        self.inlines
    }
}

pub struct ListBuild {
    items: Vec<crate::Block>,
}

impl ListBuild {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn item(mut self, f: impl FnOnce(InlineBuild) -> InlineBuild) -> Self {
        let builder = InlineBuild::new();
        let content = f(builder).build();
        self.items.push(crate::Block::Paragraph(content));
        self
    }

    pub fn build(self) -> Vec<crate::Block> {
        self.items
    }
}

pub struct TableBuild {
    headers: Vec<crate::Inline>,
    rows: Vec<Vec<crate::Inline>>,
    alignments: Vec<crate::Alignment>,
}

impl TableBuild {
    pub fn new() -> Self {
        Self {
            headers: vec![],
            rows: vec![],
            alignments: vec![],
        }
    }

    pub fn header(mut self, header: impl Into<crate::Inline>) -> Self {
        self.headers.push(header.into());
        self
    }

    pub fn row(mut self, row: impl Into<Vec<crate::Inline>>) -> Self {
        self.rows.push(row.into());
        self
    }

    pub fn alignment(mut self, alignment: crate::Alignment) -> Self {
        self.alignments.push(alignment);
        self
    }

    pub fn column(
        mut self,
        header: impl Into<crate::Inline>,
        alignment: crate::Alignment,
        cells: impl Into<Vec<crate::Inline>>,
    ) -> Self {
        self.headers.push(header.into());
        self.alignments.push(alignment);
        self.rows.push(cells.into());
        self
    }

    pub fn build(self) -> crate::Block {
        crate::Block::Table {
            headers: self.headers,
            rows: self.rows,
            alignments: self.alignments,
        }
    }
}
