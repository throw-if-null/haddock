//! Inline suppression directives: a line that holds only `doc-style-disable RULE...` or
//! `doc-style-enable RULE...` in an HTML comment.

use regex::Regex;
use std::sync::LazyLock;

use crate::{is_space, patterns, posix_regex};

// The opener is split, so that a check of this file does not read a block here.
static DIRECTIVE: LazyLock<Regex> =
    LazyLock::new(|| posix_regex(concat!("^[[:space:]]*<!", "--[[:space:]]*doc-style-")));

static END: LazyLock<Regex> = LazyLock::new(|| posix_regex("[[:space:]]*-->[[:space:]]*$"));

/// Report whether a line of code-stripped text is a directive.
pub fn is_directive(line: &str) -> bool {
    DIRECTIVE.is_match(line)
}

/// Return, for each line of code-stripped text, the set of rules that a directive disables
/// at that line. A directive line itself has the empty set. An error is one message per
/// invalid directive.
pub fn regions(lines: &[String], file: &str) -> Result<Vec<u16>, Vec<String>> {
    let mut off = 0;
    let mut result = Vec::with_capacity(lines.len());
    let mut errors = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let number = index + 1;
        if !is_directive(line) {
            result.push(off);
            continue;
        }
        result.push(0);
        // Go writes `m := end.FindStringIndex(line); if m == nil { ... }`. let-else binds
        // the value and requires the else branch to leave the loop or the function.
        let Some(end) = END.find(line) else {
            errors.push(format!("{file}:{number}: directive does not end with -->"));
            continue;
        };
        let text = line[..end.start()].trim_start_matches(is_space);
        let text = text
            .strip_prefix(concat!("<!", "--"))
            .unwrap_or(text)
            .trim_start_matches(is_space);
        let mut words = text.split(is_space).filter(|word| !word.is_empty());
        let name = words.next().unwrap_or_default();
        if name != "doc-style-disable" && name != "doc-style-enable" {
            errors.push(format!("{file}:{number}: unknown directive: {name}"));
            continue;
        }
        let mut rules = 0;
        let mut any = false;
        for word in words {
            any = true;
            if word == "all" {
                rules |= patterns::ALL;
            } else if let Some(bit) = patterns::rule_bit(word) {
                rules |= bit;
            } else {
                errors.push(format!("{file}:{number}: unknown rule: {word}"));
            }
        }
        if !any {
            rules = patterns::ALL;
        }
        if name == "doc-style-disable" {
            off |= rules;
        } else {
            off &= !rules;
        }
    }
    if errors.is_empty() {
        Ok(result)
    } else {
        Err(errors)
    }
}
