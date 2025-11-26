use docloom::prelude::*;
use docloom::{md, term};

fn usage() {
    let blocks = [
        h1("Docloom Overview"),
        p((bold("Docloom"), " turns structured blocks into Markdown.")),
        h2("Getting Started"),
        p((
            "Compose ",
            italic("inline styles"),
            " and render them together.",
        )),
        code_block("rust", "fn main() { println!(\"hello\"); }"),
        p((bold("Docloom"), " turns structured blocks into Markdown.")),
        h2("Lists"),
        ul(["Supports bullet lists", "And numbered ones"]),
        ol(["Call `doc`", "Render the output"]),
        task_list([
            (true, p("Choose block types")),
            (false, p("Render to more targets")),
        ]),
        h2("Tables"),
        table(
            (Align::right("Feature"), "Description"),
            (
                ("Tables", "Markdown alignment helpers"),
                ("Task lists", "Checkbox formatting"),
            ),
        ),
        h2("Quotes"),
        quote(p("Render nested content with ease.")),
        hr(),
        p("Generate complete documents without manual Markdown stitching."),
    ];

    let markdown = md::doc(blocks.clone());
    let terminal = term::doc(blocks);

    println!("{markdown}");
    println!("{terminal}");
}

fn block_builders() {
    use docloom::prelude::*;

    // Headers
    h1("Title");
    h2("Subtitle"); // ... through h6

    // Content
    p("Paragraph text");
    code_block("rust", "code here");
    quote(p("Quoted text"));
    hr(); // horizontal rule

    // Lists
    ul([p("Item 1"), p("Item 2")]); // unordered
    ol([p("Item 1"), p("Item 2")]); // ordered
    task_list([(true, p("Done")), (false, p("Todo"))]);

    // Tables
    table(("Col1", "Col2"), [("Row1", "Data")]);
    table(
        (
            Align::left("Left"),
            Align::center("Center"),
            Align::right("Right"),
        ),
        [("A", "B", "C")],
    );
}

fn inline_builders() {
    use docloom::prelude::*;

    text("plain text");
    bold("bold text");
    italic("italic text");
    strikethrough("struck text");
    code("inline code");
    link("text", "https://example.com");
}

fn extension_traits() {
    use docloom::prelude::*;

    "Title".h1();
    "Paragraph".p();
    "text".bold();
    "text".italic();
    "link text".link("url");
}

fn md_renderer() {
    use docloom::md::{FenceStyle, ListMarker, Style, doc};

    let style = Style {
        code_fence: FenceStyle::Tilde,     // ``` or ~~~
        list_marker: ListMarker::Asterisk, // - or *
        max_heading: 6,                    // Clamp heading levels
    };

    let _content = doc([""]).with_style(style);
}

fn term_renderer() {
    use docloom::term::{Style, doc};

    let style = Style::plain()
        .unicode_boxes(false)
        .colors(false)
        .heading_colors([
            Style::BRIGHT_CYAN,
            Style::CYAN,
            Style::BRIGHT_BLUE,
            Style::BLUE,
            Style::BRIGHT_WHITE,
            Style::BRIGHT_WHITE,
        ]);

    let _plain = doc([""]).with_style(style);
    let _ascii = doc([""]).with_style(Style::ascii());
}

fn tuple_conventions() {
    use docloom::prelude::*;

    // Multiple inline elements from tuple
    p(("Hello, ", bold("world"), "!"));

    // Table rows from nested tuples
    #[rustfmt::skip]
    table(
        ("Name", Align::right("Age")),
        (
            ("Alice", "30"),
            ("Bob", "25"),
        )
    );
}

fn custom_rendering() {
    use docloom::{Block, Inline, Render};
    use std::fmt;

    #[allow(unused)]
    struct MyRenderer;

    impl Render for MyRenderer {
        type Output = Result<(), fmt::Error>;

        fn render_block(&mut self, _block: &Block) -> Self::Output {
            // Custom block rendering
            Ok(())
        }

        fn render_inline(&mut self, _inline: &Inline) -> Self::Output {
            // Custom inline rendering
            Ok(())
        }
    }
}

fn main() {
    usage();
    block_builders();
    inline_builders();
    extension_traits();
    md_renderer();
    term_renderer();
    tuple_conventions();
    custom_rendering();
}
