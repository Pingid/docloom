/// Create a document from a list of blocks.
/// ```
/// use docloom::*;
///
/// let doc = doc![
///     h1!("Title"),
///     p!("Some text"),
///     hr!(),
/// ];
/// ```
#[macro_export]
macro_rules! doc {
    ( $( $block:expr ),* $(,)? ) => { vec![ $( ::std::convert::Into::<$crate::Block>::into($block) ),* ] };
}

// ===== Block Macros (in enum order) =====

/// Create a paragraph block.
/// ```
/// use docloom::*;
///
/// let para1 = p!("Simple paragraph");
/// let para2 = p!("Text with ", bold!("emphasis"), " and ", code!("code"));
/// ```
#[macro_export]
macro_rules! p {
    ( $( $content:expr ),* $(,)? ) => { $crate::Block::Paragraph(vec![$( ::std::convert::Into::<$crate::Inline>::into($content) ),* ]) };
}

/// Create a heading with specified level (1-6).
/// ```
/// use docloom::*;
///
/// let heading1 = h!(1, "Main Title");
/// let heading2 = h!(2, "Section ", bold!("Important"));
/// ```
#[macro_export]
macro_rules! h {
    ($level:expr, $( $content:expr ),* $(,)? ) => {
        $crate::Block::Heading { level: $level, content: vec![ $( ::std::convert::Into::<$crate::Inline>::into($content) ),* ], }
    };
}

/// Create a level 1 heading.
/// ```
/// use docloom::*;
///
/// let title = h1!("Main Title");
/// let chapter = h1!("Chapter ", code!("1"), ": Introduction");
/// ```
#[macro_export]
macro_rules! h1 { ( $( $content:expr ),* $(,)? ) => { $crate::h!(1, $( $content ),*) }; }

/// Create a level 2 heading.
/// ```
/// use docloom::*;
///
/// let section = h2!("Section Title");
/// ```
#[macro_export]
macro_rules! h2 { ( $( $content:expr ),* $(,)? ) => { $crate::h!(2, $( $content ),*) }; }

/// Create a level 3 heading.
/// ```
/// use docloom::*;
///
/// let subsection = h3!("Subsection");
/// ```
#[macro_export]
macro_rules! h3 { ( $( $content:expr ),* $(,)? ) => { $crate::h!(3, $( $content ),*) }; }

/// Create a level 4 heading.
/// ```
/// use docloom::*;
///
/// let minor = h4!("Minor Section");
/// ```
#[macro_export]
macro_rules! h4 { ( $( $content:expr ),* $(,)? ) => { $crate::h!(4, $( $content ),*) }; }

/// Create a level 5 heading.
/// ```
/// use docloom::*;
///
/// let small = h5!("Small Heading");
/// ```
#[macro_export]
macro_rules! h5 { ( $( $content:expr ),* $(,)? ) => { $crate::h!(5, $( $content ),*) }; }

/// Create a level 6 heading.
/// ```
/// use docloom::*;
///
/// let smallest = h6!("Smallest Heading");
/// ```
#[macro_export]
macro_rules! h6 { ( $( $content:expr ),* $(,)? ) => { $crate::h!(6, $( $content ),*) }; }

/// Create a code block with optional language.
/// ```
/// use docloom::*;
///
/// let code1 = code_block!("fn main() {}");
/// let code2 = code_block!("rust", "fn main() {\n    println!(\"Hello\");\n}");
/// ```
#[macro_export]
macro_rules! code_block {
    ($content:expr) => {
        $crate::Block::CodeBlock {
            language: None,
            content: $content.into(),
        }
    };
    ($language:expr, $content:expr) => {
        $crate::Block::CodeBlock {
            language: Some($language.into()),
            content: $content.into(),
        }
    };
}

/// Create an unordered list (alias for `ul!`).
/// ```
/// use docloom::*;
///
/// let items = list![
///     "First item",
///     p!("Second item with ", bold!("bold")),
///     "Third item",
/// ];
/// ```
#[macro_export]
macro_rules! list {
    ( $( $item:expr ),* $(,)? ) => {
        $crate::Block::List { ordered: false, items: vec![ $( ::std::convert::Into::<$crate::Block>::into($item) ),* ] }
    };
}

/// Create an ordered list.
/// ```
/// use docloom::*;
///
/// let steps = ol![
///     "Step 1",
///     "Step 2",
///     p!("Step 3 with ", code!("code")),
/// ];
/// ```
#[macro_export]
macro_rules! ol {
    ( $( $item:expr ),* $(,)? ) => {
        $crate::Block::List { ordered: true, items: vec![ $( ::std::convert::Into::<$crate::Block>::into($item) ),* ] }
    };
}

/// Create an unordered list.
/// ```
/// use docloom::*;
///
/// let bullets = ul![
///     "Bullet point",
///     p!("Another point with ", italic!("emphasis")),
/// ];
/// ```
#[macro_export]
macro_rules! ul {
    ( $( $item:expr ),* $(,)? ) => {
        $crate::Block::List { ordered: false, items: vec![ $( ::std::convert::Into::<$crate::Block>::into($item) ),* ] }
    };
}

/// Create a table with headers, rows, and optional alignments.
/// ```
/// use docloom::*;
///
/// let simple_table = table! {
///     headers: ["Name", "Age"],
///     rows: [
///         ["Alice", "30"],
///         ["Bob", "25"],
///     ]
/// };
///
/// let aligned_table = table! {
///     headers: ["Left", "Center", "Right"],
///     rows: [["A", "B", "C"]],
///     alignments: [Left, Center, Right]
/// };
/// ```
#[macro_export]
macro_rules! table {
    ( headers: [ $( $header:expr ),* $(,)? ], rows: [ $( [ $( $cell:expr ),* $(,)? ] ),* $(,)? ] ) => {
        {
            let headers = vec![ $( ::std::convert::Into::<$crate::Inline>::into($header) ),* ];
            let rows = vec![ $( vec![ $( ::std::convert::Into::<$crate::Inline>::into($cell) ),* ] ),* ];
            let alignments = vec![$crate::Alignment::Left; headers.len()];
            $crate::Block::Table { headers, rows, alignments }
        }
    };
    ( headers: [ $( $header:expr ),* $(,)? ], rows: [ $( [ $( $cell:expr ),* $(,)? ] ),* $(,)? ], alignments: [ $( $align:ident ),* $(,)? ] ) => {
        {
            let headers = vec![ $( ::std::convert::Into::<$crate::Inline>::into($header) ),* ];
            let rows = vec![ $( vec![ $( ::std::convert::Into::<$crate::Inline>::into($cell) ),* ] ),* ];
            let alignments = vec![ $( $crate::Alignment::$align ),* ];
            $crate::Block::Table { headers, rows, alignments }
        }
    };
}

/// Create a horizontal rule.
/// ```
/// use docloom::*;
///
/// let divider = hr!();
/// ```
#[macro_export]
macro_rules! hr {
    () => {
        $crate::Block::HorizontalRule
    };
}

// ===== Inline Macros (in enum order) =====

/// Create plain text.
/// ```
/// use docloom::*;
///
/// let msg = text!("Hello, world!");
/// ```
#[macro_export]
macro_rules! text {
    ($text:expr) => {
        $crate::Inline::Text($text.into())
    };
}

/// Create bold text.
/// ```
/// use docloom::*;
///
/// let important = bold!("Important text");
/// let mixed = bold!("Bold with ", code!("code"), " inside");
/// ```
#[macro_export]
macro_rules! bold {
    ( $( $content:expr ),* $(,)? ) => { $crate::Inline::Bold(vec![ $( ::std::convert::Into::<$crate::Inline>::into($content) ),* ]) };
}

/// Create italic text.
/// ```
/// use docloom::*;
///
/// let emphasized = italic!("Emphasized text");
/// let mixed = italic!("Italic with ", bold!("bold"), " inside");
/// ```
#[macro_export]
macro_rules! italic {
    ( $( $content:expr ),* $(,)? ) => { $crate::Inline::Italic(vec![ $( ::std::convert::Into::<$crate::Inline>::into($content) ),* ]) };
}

/// Create inline code.
/// ```
/// use docloom::*;
///
/// let snippet = code!("let x = 42;");
/// ```
#[macro_export]
macro_rules! code {
    ($content:expr) => {
        $crate::Inline::Code($content.into())
    };
}

/// Create a link with text and URL.
/// ```
/// use docloom::*;
///
/// let url_link = link!("https://example.com");
/// let text_link = link!("Example", "https://example.com");
/// let complex_link = link!([bold!("Click"), " here"], "https://example.com");
/// ```
#[macro_export]
macro_rules! link {
    ( [ $( $text:expr ),* $(,)? ], $url:expr ) => {
        $crate::Inline::Link { text: vec![ $( ::std::convert::Into::<$crate::Inline>::into($text) ),* ], url: $url.into() }
    };
    ($text:expr, $url:expr) => {
        $crate::Inline::Link { text: vec![::std::convert::Into::<$crate::Inline>::into($text)], url: $url.into() }
    };
    ($url:expr) => {
        $crate::Inline::Link { text: vec![$crate::Inline::Text($url.into())], url: $url.into() }
    };
}
