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
fn glob_match(pattern: &[char], text: &[char]) -> bool {
    let (mut p, mut t) = (0, 0);
    // The pattern position after the last `*`, and the text position where its match ends.
    let mut star: Option<(usize, usize)> = None;
    while t < text.len() {
        let next = match pattern.get(p) {
            Some('*') => {
                star = Some((p + 1, t));
                p += 1;
                continue;
            }
            Some('?') => Some(p + 1),
            Some('[') => bracket(pattern, p, text[t]),
            Some('\\') if p + 1 < pattern.len() => (pattern[p + 1] == text[t]).then_some(p + 2),
            Some(&c) => (c == text[t]).then_some(p + 1),
            None => None,
        };
        match (next, star) {
            (Some(next), _) => {
                p = next;
                t += 1;
            }
            // Backtrack: the last `*` matches one more character.
            (None, Some((star_p, star_t))) => {
                p = star_p;
                t = star_t + 1;
                star = Some((star_p, t));
            }
            (None, None) => return false,
        }
    }
    pattern[p..].iter().all(|&c| c == '*')
}

/// Match `c` against the bracket expression that starts at `pattern[start]`, and return
/// the position after the expression if `c` matches. A `[` without a closing `]` is a
/// literal `[`. A `]` directly after the opening `[`, `[!`, or `[^` is a member.
fn bracket(pattern: &[char], start: usize, c: char) -> Option<usize> {
    let mut i = start + 1;
    let negate = matches!(pattern.get(i), Some('!' | '^'));
    if negate {
        i += 1;
    }
    let first = i;
    let Some(close) = (first + 1..pattern.len()).find(|&j| pattern[j] == ']') else {
        return (c == '[').then_some(start + 1);
    };
    let mut member = false;
    while i < close {
        if i + 2 < close && pattern[i + 1] == '-' {
            member |= (pattern[i]..=pattern[i + 2]).contains(&c);
            i += 3;
        } else {
            member |= pattern[i] == c;
            i += 1;
        }
    }
    (member != negate).then_some(close + 1)
}
