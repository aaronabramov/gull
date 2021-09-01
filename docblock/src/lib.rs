/*!


[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
![Rust CI](https://github.com/aaronabramov/gull/workflows/Rust%20CI/badge.svg)

[crates-badge]: https://img.shields.io/crates/v/docblock.svg
[crates-url]: https://crates.io/crates/docblock
[docs-badge]: https://docs.rs/docblock/badge.svg
[docs-url]: https://docs.rs/docblock

`Docblock` is a crate that provides a simple API to parse and modify
dockblocks and configuration pragrmas in it.

Example:
```

use docblock::SourceFile;
use k9::snapshot;

let source = "
/*
 * @typechecks true
 * Some documentation and stuff
 */

use a::b::c;
let a = 1 + 1;
";

let mut source_file = SourceFile::from_source(source);
source_file.set_directive("dog", Some("cat"));
source_file.set_directive("hello", Some("world"));
source_file.set_directive("flow", None);
source_file.add_text("Some more documentation?");

snapshot!(
    source_file.to_source(),
"
/*
 * @typechecks true
 * @dog cat
 * @hello world
 * @flow
 * Some documentation and stuff
 * Some more documentation?
 */

use a::b::c;
let a = 1 + 1;

");
```


*/

use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

lazy_static! {
    // Regex that matches the /* */ style comment in
    // the beginning of the file.
    static ref DOCBLOCK_RE: Regex = {
        RegexBuilder::new("^\\s*/\\*\\*?(?P<block>(?:.|\\n)*\\*/)")
            .multi_line(true)
            .build()
            .unwrap()
    };
    // Regex that can match individual lines and capture anything that looks like:
    //      @some_key some value
    static ref DIRECTIVE_RE: Regex = Regex::new("^@(?P<key>\\w+)\\s+(?P<value>.*)$").unwrap();
}

#[derive(Debug, Clone)]
enum Line {
    Directive { key: String, value: Option<String> },
    Text(String),
}

// Struct that represents a source file, which contains an optional
// docblock and the rest of the file.
// Docblock values can be mutated and file can be reprinted back with
// values updated.
#[derive(Debug)]
pub struct SourceFile {
    doc_block: Vec<Line>,
    pub rest: String,
}

impl SourceFile {
    pub fn from_source(source: &str) -> Self {
        if let Some(captures) = DOCBLOCK_RE.captures(source) {
            if let Some(block) = captures.name("block") {
                // split the file into two pieces
                //      - docblock
                //      - rest of the code without docblock
                let doc_block_str = &source[block.start()..block.end()]
                    // can probably do it with the regex, but i can't figure out how
                    .trim_end_matches("*/");
                let rest = &source[block.end()..].trim_start();

                let lines = doc_block_str.split('\n').collect::<Vec<&str>>();
                let lines = lines
                    .iter()
                    // trim all the witespace around as well as the leading `*` in the beginning of each comment line
                    .map(|l| l.trim().trim_start_matches('*').trim_start())
                    .filter(|l| !l.is_empty())
                    .map(|l| {
                        if let Some(captures) = DIRECTIVE_RE.captures(l) {
                            let key = captures
                                .name("key")
                                .expect("`key` capture must be there")
                                .as_str()
                                .to_string();

                            let value = captures.name("value").map(|v| v.as_str().to_string());
                            Line::Directive { key, value }
                        } else {
                            Line::Text(l.to_string())
                        }
                    })
                    .collect::<Vec<Line>>();

                return Self {
                    doc_block: lines,
                    rest: rest.to_string(),
                };
            }
        }

        Self {
            doc_block: vec![],
            rest: source.to_string(),
        }
    }

    // Set a docblock directive. e.g. `set_directive("cat", Some("dog"));
    // will add:
    //      @cat dog
    // line to the docblock of the file
    pub fn set_directive(&mut self, key: &str, value: Option<&str>) {
        let mut to_add = Some(Line::Directive {
            key: key.to_string(),
            value: value.map(|v| v.to_string()),
        });

        let existing = self.doc_block.iter_mut().find(|l| {
            if let Line::Directive { key: k, .. } = l {
                if key == k {
                    return true;
                }
            }
            false
        });

        // If there's already a directive with the same key, replace it with a new one
        if let Some(directive @ Line::Directive { .. }) = existing {
            *directive = to_add.take().expect("must be there");
        } else {
            // otherwise insert a new one
            let mut lines = vec![];
            std::mem::swap(&mut self.doc_block, &mut lines);

            // We'll insert it after the latest directive line in the docblock to keep
            // directive grouped.
            for line in lines.into_iter().rev() {
                // If to_add is still there, and the next line (in rev order) is a directive
                // we'll take value and push it onto the vec.
                if let (Some(_), Line::Directive { .. }) = (&mut to_add, &line) {
                    self.doc_block.push(to_add.take().expect("must be there"));
                }
                // add lines in reverse order one by one
                self.doc_block.push(line);
            }

            // If it still there (e.g. docblock was empty). Add it
            if let Some(line) = to_add {
                self.doc_block.push(line);
            }

            // reverse order again
            self.doc_block.reverse();
        }
    }

    // Add text to docblock.
    pub fn add_text(&mut self, text: &str) {
        for line in text.lines() {
            self.doc_block.push(Line::Text(line.to_string()))
        }
    }

    pub fn to_source(&self) -> String {
        let mut result = String::new();

        if !self.doc_block.is_empty() {
            result.push_str("/*\n");

            for line in &self.doc_block {
                result.push_str(" * ");
                match line {
                    Line::Text(t) => result.push_str(t),
                    Line::Directive { key, value } => result.push_str(
                        format!(
                            "@{} {}",
                            key,
                            value.as_ref().map(|s| s.as_str()).unwrap_or_default()
                        )
                        .trim(),
                    ),
                }
                result.push('\n');
            }

            result.push_str(" */\n\n");
        }

        result.push_str(&self.rest);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k9::*;

    #[test]
    fn from_source() {
        let source = "
/*
 * @hello world
 * yo
 */

use a::b::c;
1 + 1";

        let mut source_file = SourceFile::from_source(source);
        source_file.set_directive("dog", Some("cat"));
        source_file.set_directive("hi", Some("hello"));
        source_file.set_directive("hello", Some("bro"));
        source_file.set_directive("ohio", None);
        source_file.add_text(
            "
Empty line followed by some text. That
spans across multiple lines.

And also hase some empty lines in between text
blocks.",
        );

        snapshot!(
            source_file.to_source(),
            "
/*
 * @hello bro
 * @dog cat
 * @hi hello
 * @ohio
 * yo
 * 
 * Empty line followed by some text. That
 * spans across multiple lines.
 * 
 * And also hase some empty lines in between text
 * blocks.
 */

use a::b::c;
1 + 1
"
        );
    }
}
