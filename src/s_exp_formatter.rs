use std::fmt::Write;

/// Format a tree-sitter S-Expression.
///
/// # Arguments
/// `input` - The S-Expression to format.
///
/// # Examples
/// ```
/// use tree_sitter_tests_formatter::format_s_expr;
///
/// let input = "(source_file (function_declaration))";
/// let expected = r#"(source_file
///   (function_declaration)
/// )
/// "#;
/// assert_eq!(format_s_expr(input), expected);
pub fn format_s_expr(input: &str) -> String {
    let mut formatted = String::new();

    let mut indent_level = 0usize;
    let indent_str = "  ";
    let s_iter = input.trim().split_inclusive(&['(', ')', ':', ' ']);
    let mut tokens: Vec<String> = Vec::new();
    for s in s_iter.clone() {
        let trim = s.trim();
        if trim.is_empty() {
            continue;
        }
        if trim.ends_with('(') {
            tokens.push(s.replace('(', "").trim().to_string());
            tokens.push("(".to_string());
            continue;
        }
        if trim.ends_with(')') {
            tokens.push(s.replace(')', "").trim().to_string());
            tokens.push(")".to_string());
            continue;
        }
        tokens.push(trim.to_string());
    }
    tokens.retain(|s| !s.is_empty());
    let tokens_iter: std::slice::Iter<'_, String> = tokens.iter();

    // Breaks and indentation are dealt with by the ) token and identifiers.
    for (i, token) in tokens_iter.enumerate() {
        if token == "(" {
            write!(formatted, "(").unwrap();
            let next_token = tokens[i + 2].clone();
            if next_token != ")" {
                indent_level += 1;
            }
            continue;
        }
        if token == ")" {
            let prev_token = tokens[i - 2].clone();
            let next_token = tokens.get(i + 1).cloned().unwrap_or_default();
            if prev_token == "(" {
                writeln!(formatted, ")").unwrap();
                if next_token == "(" {
                    write!(formatted, "{}", indent_str.repeat(indent_level)).unwrap();
                } else {
                    write!(
                        formatted,
                        "{}",
                        indent_str.repeat(indent_level.saturating_sub(1))
                    )
                    .unwrap();
                }
            } else {
                writeln!(formatted, ")").unwrap();
                indent_level = indent_level.saturating_sub(1);
                if next_token == "(" {
                    write!(formatted, "{}", indent_str.repeat(indent_level)).unwrap();
                } else {
                    write!(
                        formatted,
                        "{}",
                        indent_str.repeat(indent_level.saturating_sub(1))
                    )
                    .unwrap();
                }
            }
            continue;
        }
        if token.ends_with(':') {
            let prev_token = tokens[i - 1].clone();
            if matches!(prev_token.as_str(), "(" | ")") || prev_token.ends_with(':') {
                write!(formatted, "  ").unwrap();
            }
            write!(formatted, "{}", token).unwrap();
            write!(formatted, " ").unwrap();
            continue;
        }
        if tokens[i + 1] == ")" {
            write!(formatted, "{}", token).unwrap();
            continue;
        }
        writeln!(formatted, "{}", token).unwrap();
        write!(formatted, "{}", indent_str.repeat(indent_level)).unwrap();
    }

    formatted
}
