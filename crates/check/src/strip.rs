//! Reduce a file to the text that the rules apply to. Each function returns one string per
//! input line, so that a line number of the result is a line number of the file. An inline
//! code span becomes `CODE`, so that it still counts as one word.

use regex::Regex;
use std::sync::LazyLock;

use crate::{directive, is_space, posix_regex, split_lines};

static CODE_SPAN: LazyLock<Regex> = LazyLock::new(|| posix_regex("`[^`]*`"));

static FENCE_OPEN: LazyLock<Regex> = LazyLock::new(|| posix_regex("^[[:space:]]*(```+|~~~+)"));

static FENCE_CLOSE: LazyLock<Regex> =
    LazyLock::new(|| posix_regex("^[[:space:]]*(```+|~~~+)[[:space:]]*$"));

static LINE_COMMENT: LazyLock<Regex> =
    LazyLock::new(|| posix_regex("^[[:space:]]*(//+|#+|--+|;+)"));

// A Lua long comment opener at the start of a line. It is not a line comment. The opener
// is split, so that a check of this file does not read a block here.
static LONG_COMMENT: LazyLock<Regex> =
    LazyLock::new(|| posix_regex(concat!(r"^[[:space:]]*-", r"-\[\[")));

static PREPROCESSOR: LazyLock<Regex> = LazyLock::new(|| {
    posix_regex(
        "^[[:space:]]*(include|define|undef|if|ifdef|ifndef|elif|else|endif|pragma|error|warning|line)([^[:alnum:]_]|$)",
    )
});

const HTML_OPEN: &str = concat!("<!", "--");

/// Each block comment form: the opener and the closer. The openers are split, so that a
/// check of this file does not read a block here.
const BLOCKS: [(&str, &str); 4] = [
    (concat!("\"\"", "\""), concat!("\"\"", "\"")),
    (concat!("''", "'"), concat!("''", "'")),
    (HTML_OPEN, "-->"),
    (concat!("-", "-[["), "]]"),
];

const C_OPEN: &str = "/*";
const C_CLOSE: &str = "*/";

/// Return the Markdown text of `text` that the rules apply to. Front matter, fenced and
/// indented code blocks, and HTML comments other than directives are blank lines.
///
/// A fence is three or more backticks or tildes. Only a line that holds the same character
/// at least as many times, and nothing else, closes it. An indented code block starts with
/// four spaces or a tab after a blank line. It ends at a line that is not blank and not
/// indented.
pub fn markdown(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut blank = true;
    let mut front = false;
    // The opening fence, without whitespace, while a fenced block is open.
    let mut fence: Option<String> = None;
    let mut indented = false;
    let mut comment = false;
    for (index, line) in split_lines(text).into_iter().enumerate() {
        let after_blank = blank;
        blank = line.chars().all(is_space);
        let delimiter = line
            .strip_prefix("---")
            .is_some_and(|rest| rest.chars().all(is_space));
        if index == 0 && delimiter {
            front = true;
            out.push(String::new());
            continue;
        }
        if front {
            front = !delimiter;
            out.push(String::new());
            continue;
        }
        if let Some(open) = &fence {
            if FENCE_CLOSE.is_match(line) {
                let mark: String = line.chars().filter(|&c| !is_space(c)).collect();
                if mark.starts_with(&open[..1]) && mark.len() >= open.len() {
                    fence = None;
                }
            }
            out.push(String::new());
            continue;
        }
        let code_line = line.starts_with("    ") || line.starts_with('\t');
        if indented && (blank || code_line) {
            out.push(String::new());
            continue;
        }
        indented = false;
        if comment {
            match line.find("-->") {
                None => out.push(String::new()),
                Some(end) => {
                    comment = false;
                    out.push(emit(&line[end + 3..], &mut comment));
                }
            }
            continue;
        }
        if let Some(open) = FENCE_OPEN.find(line) {
            fence = Some(open.as_str().chars().filter(|&c| !is_space(c)).collect());
            out.push(String::new());
            continue;
        }
        if after_blank && code_line {
            indented = true;
            out.push(String::new());
            continue;
        }
        if directive::is_directive(line) {
            out.push(line.to_string());
            continue;
        }
        out.push(emit(line, &mut comment));
    }
    out
}

/// Return `text` with inline code spans reduced to `CODE` and HTML comments removed. A
/// comment without `-->` continues on the next line, and `comment` is then true.
// `&mut bool` is an out parameter, as a `*bool` in Go. The borrow checker allows no other
// reference to the flag while emit runs.
fn emit(text: &str, comment: &mut bool) -> String {
    // replace_all returns a Cow: the input unchanged when nothing matches, or a new
    // String. into_owned allocates only in the first case.
    let mut text = CODE_SPAN.replace_all(text, "CODE").into_owned();
    while let Some(start) = text.find(HTML_OPEN) {
        let rest = &text[start + HTML_OPEN.len()..];
        let Some(end) = rest.find("-->") else {
            text.truncate(start);
            *comment = true;
            break;
        };
        text = format!("{} {}", &text[..start], &rest[end + 3..]);
    }
    text
}

/// Return the comment text of each line of source code `text`, without the comment
/// markers. A line without a comment is blank.
///
/// A comment is a line that starts with `//`, `#`, `--`, or `;`. A block between an opener
/// and a closer of `BLOCKS`, or between `/*` and `*/`, is also a comment. A `/*` opens a
/// block only at the start of a line or after whitespace. A `#` or `//` after code starts a
/// comment when two conditions hold: whitespace precedes it, and the `"` and `'` characters
/// before it are both even in number. A `#` line that starts with a C preprocessor
/// directive name is not a comment.
pub fn comments(text: &str) -> Vec<String> {
    // The closer of the block comment that is open at the start of the line.
    let mut block: Option<&str> = None;
    let mut result = Vec::new();
    for whole in split_lines(text) {
        let mut line = whole;
        let mut out = String::new();
        // A let chain: the `if let` binds only when every condition before it holds.
        if block.is_none()
            && !LONG_COMMENT.is_match(line)
            && let Some(marker) = LINE_COMMENT.find(line)
        {
            out = line[marker.end()..].to_string();
            if marker.as_str().ends_with('#') && PREPROCESSOR.is_match(&out) {
                out.clear();
            }
            line = "";
        }
        while !line.is_empty() {
            if let Some(closer) = block {
                let Some(end) = line.find(closer) else {
                    piece(&mut out, line, closer);
                    break;
                };
                piece(&mut out, &line[..end], closer);
                line = &line[end + closer.len()..];
                block = None;
                continue;
            }
            // The first opener on the line: its position, its length, and its closer.
            let mut open: Option<(usize, usize, &str)> = None;
            let candidates = BLOCKS
                .iter()
                .filter_map(|&(opener, closer)| Some((line.find(opener)?, opener.len(), closer)))
                .chain(c_block_start(line).map(|start| (start, C_OPEN.len(), C_CLOSE)));
            for candidate in candidates {
                if open.is_none_or(|(start, _, _)| candidate.0 < start) {
                    open = Some(candidate);
                }
            }
            if let Some(start) = trailing(whole, line)
                && open.is_none_or(|(open_start, _, _)| start < open_start)
            {
                let text = &line[start..];
                let marker = if text.starts_with('#') { '#' } else { '/' };
                out.push(' ');
                out.push_str(text.trim_start_matches(marker));
                break;
            }
            let Some((start, length, closer)) = open else {
                break;
            };
            line = &line[start + length..];
            block = Some(closer);
        }
        result.push(CODE_SPAN.replace_all(&out, "CODE").into_owned());
    }
    result
}

/// Append the text of a block comment to `out`. In a `/* */` block, a leading `*` is
/// decoration, not text.
fn piece(out: &mut String, text: &str, closer: &str) {
    let mut text = text;
    if closer == C_CLOSE {
        let rest = text.trim_start_matches(is_space);
        if rest.starts_with('*') {
            text = rest.trim_start_matches('*');
        }
    }
    out.push(' ');
    out.push_str(text);
}

/// Return the position of the first `/*` in `line` that opens a block: at the start of the
/// line or after whitespace. A `/*` inside a word, as in a path glob, does not open a block.
fn c_block_start(line: &str) -> Option<usize> {
    let mut from = 0;
    while let Some(found) = line[from..].find(C_OPEN) {
        let start = from + found;
        if start == 0 || line[..start].ends_with(is_space) {
            return Some(start);
        }
        from = start + C_OPEN.len();
    }
    None
}

/// Return the position in `line` of the first `#` or `//` that starts a comment after
/// code. `line` is a suffix of `whole`, and the conditions apply to `whole`: whitespace
/// precedes the marker, and the `"` and `'` characters before it are both even in number.
fn trailing(whole: &str, line: &str) -> Option<usize> {
    let offset = whole.len() - line.len();
    let mut from = 0;
    loop {
        // `?` on an Option returns None from the function when no marker is left.
        let start = from + line[from..].find(['#', '/'])?;
        from = start + 1;
        if !line[start..].starts_with('#') && !line[start..].starts_with("//") {
            continue;
        }
        let before = &whole[..offset + start];
        if !before.ends_with(is_space) {
            continue;
        }
        if before.matches('"').count() % 2 == 1 || before.matches('\'').count() % 2 == 1 {
            continue;
        }
        return Some(start);
    }
}

// Go keeps tests in a _test.go file of the same package. A Rust unit test module is a child
// of the module it tests, and cfg(test) compiles it only for `cargo test`.
#[cfg(test)]
mod tests {
    use super::markdown;

    #[test]
    fn keeps_one_line_per_input_line() {
        assert_eq!(markdown("a\r\n\nb"), ["a\r", "", "b"]);
        assert_eq!(markdown("a\n"), ["a"]);
        assert_eq!(markdown(""), Vec::<String>::new());
    }

    #[test]
    fn blanks_front_matter_on_line_one_only() {
        assert_eq!(
            markdown("---\nkey: just\n---\nText."),
            ["", "", "", "Text."]
        );
        assert_eq!(
            markdown("Text.\n---\nkey: just"),
            ["Text.", "---", "key: just"]
        );
    }

    #[test]
    fn blanks_a_fenced_block_until_a_closing_fence() {
        assert_eq!(markdown("```bash\njust\n```\nText."), ["", "", "", "Text."]);
        assert_eq!(
            markdown("~~~\n```\njust\n~~~\nText."),
            ["", "", "", "", "Text."]
        );
    }

    #[test]
    fn closes_a_fence_only_with_as_many_marks_and_no_info_string() {
        let text = "````\n```\n```text\njust\n````\nText.";
        assert_eq!(markdown(text), ["", "", "", "", "", "Text."]);
    }

    #[test]
    fn blanks_an_indented_block_after_a_blank_line() {
        let text = "Text.\n\n    just\n\n\tjust\nText.";
        assert_eq!(markdown(text), ["Text.", "", "", "", "", "Text."]);
    }

    #[test]
    fn keeps_an_indented_line_after_text() {
        assert_eq!(markdown("- item\n    wrapped"), ["- item", "    wrapped"]);
    }

    #[test]
    fn reduces_an_inline_code_span_to_code() {
        assert_eq!(markdown("Run `just build` now."), ["Run CODE now."]);
    }

    #[test]
    fn removes_an_html_comment_on_one_line() {
        assert_eq!(markdown("a <!-- removed --> b"), ["a   b"]);
    }

    #[test]
    fn removes_an_html_comment_on_several_lines() {
        let text = "a <!-- removed\nremoved\n--> b";
        assert_eq!(markdown(text), ["a ", "", " b"]);
    }

    #[test]
    fn keeps_a_directive_outside_code() {
        let directive = "<!-- doc-style-disable tone -->";
        assert_eq!(markdown(directive), [directive]);
        let fenced = format!("```\n{directive}\n```");
        assert_eq!(markdown(&fenced), ["", "", ""]);
    }
}
