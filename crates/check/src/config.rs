//! The `.doc-style` project file: rules disabled by path pattern, and allowed words.

use std::fs;
use std::path::{Path, PathBuf};

use crate::{patterns, split_lines};

/// The settings that one `.doc-style` file gives one checked file.
// derive(Default) writes the zero value: no rule off, no allowed word. Go has a zero
// value for every type without a declaration.
#[derive(Default)]
pub struct Config {
    /// The rules that a pattern line disables for the file.
    pub off: u16,
    /// The allowed words, in lower case.
    allow: Vec<String>,
}

impl Config {
    /// Report whether the whole text of a match is an allowed word, ignoring case.
    pub fn allows(&self, matched: &str) -> bool {
        let matched = matched.to_lowercase();
        self.allow.contains(&matched)
    }
}

/// Return the nearest `.doc-style` file in the directory of `file` or above it. The search
/// stops at the root of the git repository that holds `file`. A worktree or a submodule
/// has a `.git` file, not a directory, so any `.git` entry marks the root.
pub fn find(file: &Path) -> Option<PathBuf> {
    let parent = file
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    // canonicalize resolves symbolic links, as `pwd -P` does.
    let start = fs::canonicalize(parent).ok()?;
    // ancestors yields the path, then each parent up to the root. Go has no such iterator:
    // a loop calls filepath.Dir until the result stops changing.
    for dir in start.ancestors() {
        let candidate = dir.join(".doc-style");
        if candidate.is_file() {
            return Some(candidate);
        }
        if dir.join(".git").exists() {
            return None;
        }
    }
    None
}

/// Read `config` for `file`. A line is a path pattern and one or more rule IDs, or `allow`
/// and one or more words. A line that starts with `#` is a comment. An error is one
/// message per invalid line.
pub fn read(config: &Path, file: &Path) -> Result<Config, Vec<String>> {
    let shown = config.display();
    let text = fs::read(config).map_err(|error| vec![format!("{shown}: {error}")])?;
    let text = String::from_utf8_lossy(&text);
    let relative = relative_path(config, file);
    let relative: Vec<char> = relative.chars().collect();
    let mut result = Config::default();
    let mut errors = Vec::new();
    for (index, line) in split_lines(&text).into_iter().enumerate() {
        let number = index + 1;
        // bash `read -a` splits at spaces and tabs only. A carriage return stays in a word.
        let mut words = line.split([' ', '\t']).filter(|word| !word.is_empty());
        let Some(pattern) = words.next() else {
            continue;
        };
        if pattern.starts_with('#') {
            continue;
        }
        let ids: Vec<&str> = words.collect();
        if pattern == "allow" {
            if ids.is_empty() {
                errors.push(format!("{shown}:{number}: no word after allow"));
            }
            result
                .allow
                .extend(ids.iter().map(|word| word.to_lowercase()));
            continue;
        }
        if ids.is_empty() {
            errors.push(format!(
                "{shown}:{number}: no rule after pattern: {pattern}"
            ));
            continue;
        }
        let pattern: Vec<char> = pattern.chars().collect();
        let matched = glob_match(&pattern, &relative);
        for id in ids {
            let bit = if id == "all" {
                patterns::ALL
            } else if let Some(bit) = patterns::rule_bit(id) {
                bit
            } else {
                errors.push(format!("{shown}:{number}: unknown rule: {id}"));
                continue;
            };
            if matched {
                result.off |= bit;
            }
        }
    }
    if errors.is_empty() {
        Ok(result)
    } else {
        Err(errors)
    }
}

/// Return the path of `file` relative to the directory of `config`, with symbolic links in
/// the directory of `file` resolved.
fn relative_path(config: &Path, file: &Path) -> String {
    let parent = file
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let name = file.file_name().unwrap_or_default();
    let absolute = fs::canonicalize(parent).unwrap_or_default().join(name);
    let base = config.parent().unwrap_or(Path::new("/"));
    let relative = absolute.strip_prefix(base).unwrap_or(&absolute);
    relative.to_string_lossy().into_owned()
}

/// Match `text` against a bash pattern, as `[[ $text == $pattern ]]` does. `*` matches any
/// string, `/` included. `?` matches one character. `[...]` matches one character of a
/// set, and `[!...]` or `[^...]` one character outside it. `\` quotes the next character.
///
/// The match backtracks at each `*`, so the time grows exponentially with the number of
/// `*` characters. A `.doc-style` pattern holds few of them.
fn glob_match(pattern: &[char], text: &[char]) -> bool {
    // A slice pattern matches the shape of a slice and binds its parts. `rest @ ..` binds
    // the remaining elements. Go has no equivalent, and indexes the slice instead.
    if let ['*', rest @ ..] = pattern {
        return glob_match(rest, text)
            || text
                .split_first()
                .is_some_and(|(_, text)| glob_match(pattern, text));
    }
    let Some((&c, text)) = text.split_first() else {
        return pattern.is_empty();
    };
    let rest = match pattern {
        ['?', rest @ ..] => Some(rest),
        ['[', ..] => bracket(pattern, c),
        ['\\', quoted, rest @ ..] => (*quoted == c).then_some(rest),
        [literal, rest @ ..] => (*literal == c).then_some(rest),
        [] => None,
    };
    rest.is_some_and(|rest| glob_match(rest, text))
}

/// Match `c` against the bracket expression at the start of `pattern`, and return the rest
/// of the pattern after the expression if `c` matches. A `[` without a closing `]` is a
/// literal `[`. A `]` directly after the opening `[`, `[!`, or `[^` is a member.
fn bracket(pattern: &[char], c: char) -> Option<&[char]> {
    let after_open = &pattern[1..];
    let (negate, body) = match after_open {
        ['!' | '^', body @ ..] => (true, body),
        body => (false, body),
    };
    // The search for the closing `]` skips the first member.
    let Some(close) = body.iter().skip(1).position(|&x| x == ']') else {
        return (c == '[').then_some(after_open);
    };
    let (members, rest) = body.split_at(close + 1);
    (in_set(members, c) != negate).then_some(&rest[1..])
}

/// Report whether `c` is a member of the set of a bracket expression. `a-z` is a range.
fn in_set(members: &[char], c: char) -> bool {
    match members {
        [] => false,
        [low, '-', high, rest @ ..] => (*low..=*high).contains(&c) || in_set(rest, c),
        [member, rest @ ..] => *member == c || in_set(rest, c),
    }
}

#[cfg(test)]
mod tests {
    use super::glob_match;

    /// Match `text` against `pattern` with glob_match. Each expected result in the tests is
    /// the result of `[[ $text == $pattern ]]` in bash.
    fn glob(pattern: &str, text: &str) -> bool {
        let pattern: Vec<char> = pattern.chars().collect();
        let text: Vec<char> = text.chars().collect();
        glob_match(&pattern, &text)
    }

    #[test]
    fn matches_any_string_with_a_star() {
        assert!(glob("*.md", "docs/a.md"));
        assert!(glob("docs/*", "docs/"));
        assert!(glob("*", ""));
        assert!(glob("a*b*c", "axbxxc"));
        assert!(!glob("a*b*c", "axbxx"));
        assert!(!glob("*.md", "a.mdx"));
    }

    #[test]
    fn matches_one_character_with_a_question_mark() {
        assert!(glob("a?c", "a/c"));
        assert!(!glob("a?c", "ac"));
        assert!(!glob("?", ""));
    }

    #[test]
    fn matches_one_character_of_a_bracket_expression() {
        assert!(glob("[ab]", "b"));
        assert!(!glob("[ab]", "c"));
        assert!(glob("[a-c]x", "bx"));
        assert!(!glob("[a-c]", "d"));
        assert!(glob("[a-]", "-"));
        assert!(glob("[]a]", "]"));
    }

    #[test]
    fn matches_one_character_outside_a_negated_bracket_expression() {
        assert!(glob("[!ab]", "c"));
        assert!(!glob("[!ab]", "a"));
        assert!(glob("[^ab]", "c"));
        assert!(!glob("[!]a]", "]"));
    }

    #[test]
    fn reads_an_unterminated_bracket_as_a_literal() {
        assert!(glob("[ab", "[ab"));
        assert!(!glob("[ab", "a"));
        assert!(glob("[!", "[!"));
    }

    #[test]
    fn quotes_the_next_character_with_a_backslash() {
        assert!(glob(r"\*", "*"));
        assert!(!glob(r"\*", "a"));
        assert!(glob(r"a\", r"a\"));
    }

    #[test]
    fn requires_a_match_of_the_whole_text() {
        assert!(!glob("a", "ab"));
        assert!(!glob("ab", "a"));
    }
}
