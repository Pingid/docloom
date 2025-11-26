use docloom::prelude::*;
use docloom::{md, term};

fn main() {
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
            ("Feature", "Description"),
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
