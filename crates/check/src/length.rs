//! The length rule: the sentences of code-stripped text, with their word counts.

use regex::Regex;
use std::sync::LazyLock;

use crate::{is_space, posix_regex};

// A terminator may carry closing emphasis, a backtick, a quote, or a bracket before the
// space. A cut on the bare terminator merges a bold lead-in sentence with the next one.
static TERMINATOR: LazyLock<Regex> =
    LazyLock::new(|| posix_regex(r#"[.!?:][*`")\]]*[[:space:]]+"#));

static LIST_MARKER: LazyLock<Regex> =
    LazyLock::new(|| posix_regex(r"^[[:space:]]*([-*+]|[0-9]+[.)])[[:space:]]+"));

/// One sentence of a paragraph.
#[derive(Debug, PartialEq)]
pub struct Sentence {
    /// The line where the sentence starts. The first line is 1.
    pub line: usize,
    /// The number of words.
    pub words: usize,
    /// The sentence, without leading and trailing whitespace.
    pub text: String,
}

/// Split code-stripped lines into sentences. The lines of a paragraph are joined, because
/// a sentence spans several wrapped lines. A blank line, a heading, a table row, or a
/// blockquote ends a paragraph, and is not part of one.
///
/// A list marker starts a new paragraph, and the marker is not a word. Without that rule,
/// a list of short items is one long sentence. A wrapped continuation line carries no
/// marker, so it joins the item above it.
// `impl AsRef<str>` accepts a slice of String or of &str. Go needs one concrete type, or a
// type parameter with a constraint.
pub fn sentences(lines: &[impl AsRef<str>]) -> Vec<Sentence> {
    let mut paragraph = Paragraph::default();
    let mut result = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let line = line.as_ref();
        let number = index + 1;
        let rest = line.trim_start_matches(is_space);
        if rest.is_empty() || rest.starts_with(['#', '|', '>']) {
            paragraph.flush(&mut result);
        } else if let Some(marker) = LIST_MARKER.find(line) {
            paragraph.flush(&mut result);
            paragraph.add(&line[marker.end()..], number);
        } else {
            paragraph.add(line, number);
        }
    }
    paragraph.flush(&mut result);
    result
}

/// The lines of one paragraph, joined with spaces.
#[derive(Default)]
struct Paragraph {
    text: String,
    /// The line where each sentence starts: one entry for the first sentence, and one for
    /// each terminator.
    starts: Vec<usize>,
}

// Methods go in an impl block, apart from the struct. Go declares each method with its
// receiver at the top level.
impl Paragraph {
    /// Append a line. A new terminator starts a sentence on this line. A terminator at the
    /// end of a line takes effect when the next line adds the joining space. The next
    /// sentence then starts on that line.
    fn add(&mut self, line: &str, number: usize) {
        if self.text.is_empty() {
            self.starts = vec![number];
        }
        self.text.push(' ');
        self.text.push_str(line.trim_end_matches(is_space));
        let terminators = TERMINATOR.find_iter(&self.text).count();
        // The count never drops when a line is appended, so resize only appends here.
        self.starts.resize(terminators + 1, number);
    }

    /// Move the sentences of the paragraph to `result`, and start an empty paragraph.
    fn flush(&mut self, result: &mut Vec<Sentence>) {
        // mem::take moves the String out and leaves an empty one in its place.
        let text = std::mem::take(&mut self.text);
        let ends = TERMINATOR
            .find_iter(&text)
            .map(|terminator| terminator.end())
            .chain([text.len()]);
        let mut start = 0;
        for (end, &line) in ends.zip(&self.starts) {
            let sentence = text[start..end].trim_matches(is_space);
            start = end;
            let words = sentence
                .split(is_space)
                .filter(|word| !word.is_empty())
                .count();
            if words > 0 {
                result.push(Sentence {
                    line,
                    words,
                    text: sentence.to_string(),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sentences;

    /// Return the start line and the word count of each sentence.
    fn split(lines: &[&str]) -> Vec<(usize, usize)> {
        sentences(lines)
            .into_iter()
            .map(|sentence| (sentence.line, sentence.words))
            .collect()
    }

    #[test]
    fn cuts_at_each_terminator_followed_by_whitespace() {
        assert_eq!(
            split(&["One two. Three! Four? Five: six seven."]),
            [(1, 2), (1, 1), (1, 1), (1, 1), (1, 2)]
        );
        assert_eq!(split(&["Version 1.2 is out."]), [(1, 4)]);
    }

    #[test]
    fn joins_wrapped_lines_and_reports_the_start_line() {
        assert_eq!(
            split(&["One two", "three. Four", "five six."]),
            [(1, 3), (2, 3)]
        );
    }

    #[test]
    fn starts_a_sentence_after_a_line_end_terminator_on_the_next_line() {
        assert_eq!(split(&["One.", "Two three."]), [(1, 1), (2, 2)]);
    }

    #[test]
    fn cuts_after_closing_emphasis_and_quotes() {
        assert_eq!(split(&["**Lead in.** Rest of it."]), [(1, 2), (1, 3)]);
        assert_eq!(split(&["He said \"stop.\" Then left."]), [(1, 3), (1, 2)]);
    }

    #[test]
    fn ends_a_paragraph_at_a_blank_line() {
        assert_eq!(split(&["One two", "", "three four"]), [(1, 2), (3, 2)]);
    }

    #[test]
    fn excludes_a_heading_a_table_row_and_a_blockquote() {
        let lines = [
            "One",
            "# Heading",
            "two",
            "| a | b |",
            "three",
            "> quote",
            "four",
        ];
        assert_eq!(split(&lines), [(1, 1), (3, 1), (5, 1), (7, 1)]);
    }

    #[test]
    fn starts_a_paragraph_at_a_list_marker_and_does_not_count_it() {
        let lines = [
            "- one two",
            "  three",
            "1. four",
            "2) five",
            "* six",
            "+ seven",
        ];
        assert_eq!(split(&lines), [(1, 3), (3, 1), (4, 1), (5, 1), (6, 1)]);
    }

    #[test]
    fn does_not_end_a_sentence_at_trailing_whitespace() {
        assert_eq!(split(&["One two  ", "three."]), [(1, 3)]);
    }

    #[test]
    fn does_not_count_a_unicode_space_as_a_word() {
        assert_eq!(split(&["one \u{3000} two \u{2003} three"]), [(1, 3)]);
        assert_eq!(split(&["one \u{a0} two"]), [(1, 3)]);
    }

    #[test]
    fn keeps_the_sentence_text_without_surrounding_whitespace() {
        let texts: Vec<String> = sentences(&["  One two.   Three  "])
            .into_iter()
            .map(|sentence| sentence.text)
            .collect();
        assert_eq!(texts, ["One two.", "Three"]);
    }
}
