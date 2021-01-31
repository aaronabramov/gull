pub enum CommentStyle {
    DoubleSlash,
    TripleSlash,
}

pub fn format_docstring(doc: &str, style: CommentStyle, indent: usize) -> Option<String> {
    if doc.trim().is_empty() {
        return None;
    }

    let doc = adjust_indentation(doc);

    let indent = " ".repeat(indent);

    let prepend = match style {
        CommentStyle::DoubleSlash => format!("{}// ", indent),
        CommentStyle::TripleSlash => format!("{}/// ", indent),
    };

    let mut result = vec![];

    for line in doc.trim().lines() {
        result.push(format!("{}{}", prepend, line));
    }

    Some(result.join("\n"))
}

/// Adjust indentation of the whole block to allow for
/// indented doc strings.
/// e.g.
///
/// MyStruct {
///     multiline_string: "hello
///     world
///     this is a multiline string",
/// }
///
///
/// will result is a string that is indented like:
/// "hello\n     world\n     this is a multiline string"
/// and will look super awkward when codegened as:
///
/// // hello
/// //   this is
/// //   a multiline string
///
///
/// This function finds the minimun amount we can "de-indent" the block
/// and trims the beginning of each lines whitespace to achieve that.
fn adjust_indentation(doc: &str) -> String {
    let mut lines = doc.trim().lines();

    // first line is usually not indented, since it starts right after "
    // e.g. my_str = "first line
    // second line
    // third line"
    lines.next();

    let mut min_line_indent = None;

    for line in lines {
        let mut chars = line.chars();
        if line.trim().is_empty() {
            continue;
        }

        let mut line_indent = 0;

        while let Some(' ') = chars.next() {
            line_indent += 1;
        }

        if let Some(current_min) = min_line_indent {
            min_line_indent = Some(std::cmp::min(line_indent, current_min));
        } else {
            min_line_indent = Some(line_indent);
        }
    }

    dbg!(min_line_indent);
    match min_line_indent {
        None | Some(0) => doc.to_string(), // no changes needed
        Some(indent_to_trim) => {
            let prefix_to_trim = " ".repeat(indent_to_trim);

            doc.lines()
                .map(|l| l.strip_prefix(&prefix_to_trim).unwrap_or(l))
                .collect::<Vec<&str>>()
                .join("\n")
        }
    }
}
