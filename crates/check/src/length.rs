//! The length rule: the sentences of code-stripped text, with their word counts.

use regex::Regex;
use std::sync::LazyLock;

use crate::is_space;

// A terminator may carry closing emphasis, a backtick, a quote, or a bracket before the
// space. A cut on the bare terminator merges a bold lead-in sentence with the next one.
static TERMINATOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[.!?:][*`")\]]*[[:space:]]+"#).unwrap());

static LIST_MARKER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[[:space:]]*([-*+]|[0-9]+[.)])[[:space:]]+").unwrap());

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
        while self.starts.len() <= terminators {
            self.starts.push(number);
        }
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
