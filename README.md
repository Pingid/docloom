# docloom

A Rust library for programmatically building and rendering documents to markdown and styled terminal output.

## Installation

```toml
[dependencies]
docloom = "0.1.0"
```

## Usage

```rust
use docloom::{markdown, terminal};
use docloom::prelude::*;

let doc = vec![
    h1("Title"),
    p("A paragraph with some text."),
    ul([
        p("First item"),
        p("Second item"),
    ]),
    code_block(Some("rust".into()), "let x = 42;"),
];

// Render to markdown
let md = markdown::Renderer::to_string(&doc[..]);
println!("{}", md);

// Render to terminal with colors
let term = terminal::Renderer::to_string(&doc[..]);
println!("{}", term);
```

## Document Structure

Documents are built from `Block` and `Inline` elements.

### Block Elements

- `Paragraph(Vec<Inline>)` - Text paragraph
- `Heading { level, content }` - Headers (h1-h6)
- `CodeBlock { language, content }` - Fenced code blocks
- `List { ordered, items }` - Ordered/unordered lists
- `TaskList { items }` - Checkbox lists
- `Table { headers, rows, alignments }` - Tables with alignment
- `Blockquote(Vec<Block>)` - Quoted blocks
- `HorizontalRule` - Horizontal divider
- `BlockList(Vec<Block>)` - Container for multiple blocks

### Inline Elements

- `Text(String)` - Plain text
- `Bold(Vec<Inline>)` - **Bold** formatting
- `Italic(Vec<Inline>)` - _Italic_ formatting
- `Strikethrough(Vec<Inline>)` - ~~Strikethrough~~ formatting
- `Code(String)` - `Inline code`
- `Link { text, url }` - Hyperlinks
- `LineBreak` - Line break

## Builder Functions

### Block Builders

```rust
// Headers
h1("Title")
h2("Subtitle")  // ... through h6

// Content
p("Paragraph text")
code_block(Some("rust".into()), "code here")
quote([p("Quoted text")])
hr()  // horizontal rule

// Lists
ul([p("Item 1"), p("Item 2")])  // unordered
ol([p("Item 1"), p("Item 2")])  // ordered
task_list([(true, p("Done")), (false, p("Todo"))])

// Tables
table(("Col1", "Col2"), [("Row1", "Data")])
table_aligned(
    ("Left", "Center", "Right"),
    [("A", "B", "C")],
    vec![Alignment::Left, Alignment::Center, Alignment::Right]
)
```

### Inline Builders

```rust
text("plain text")
bold("bold text")
italic("italic text")
strikethrough("struck text")
code("inline code")
link("text", "https://example.com")
```

## Extension Traits

Use method syntax with `BlockExt` and `InlineExt`:

```rust
"Title".h1()
"Paragraph".p()
"text".bold()
"text".italic()
"link text".link("url")
```

## Renderers

### Markdown Renderer

Outputs standard markdown with configurable styles:

````rust
use docloom::markdown::{Renderer, Style, FenceStyle, ListMarker};

let style = Style {
    code_fence: FenceStyle::Backtick,  // ``` or ~~~
    list_marker: ListMarker::Dash,     // - or *
    max_heading: 6,                     // Clamp heading levels
};

let output = Renderer::to_string_with_style(&doc, style);
````

### Terminal Renderer

Outputs styled terminal output with ANSI codes:

```rust
use docloom::terminal::{Renderer, Style};

// Default: colors and unicode
let output = Renderer::to_string(&doc);

// Plain text (no colors)
let plain = Renderer::to_string_with_style(&doc, Style::plain());

// ASCII-only (no unicode)
let ascii = Renderer::to_string_with_style(&doc, Style::ascii());
```

Terminal features:

- Colored headers, code, links
- Unicode or ASCII box drawing
- Table alignment support
- Indented lists and blockquotes

## Tuple Convenience

Build content from tuples for concise syntax:

```rust
// Multiple inline elements from tuple
p(("Hello, ", bold("world"), "!"))

// Table rows from nested tuples
table(
    ("Name", "Age"),
    (
        ("Alice", "30"),
        ("Bob", "25"),
    )
)
```

## Custom Rendering

Implement the `Render` trait for custom output formats:

```rust
impl<W: fmt::Write> Render for MyRenderer<W> {
    type Output = Result<(), fmt::Error>;

    fn render_block(&mut self, block: &Block) -> Self::Output {
        // Custom block rendering
    }

    fn render_inline(&mut self, inline: &Inline) -> Self::Output {
        // Custom inline rendering
    }
}
```

## License

MIT
