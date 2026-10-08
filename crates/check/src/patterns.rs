//! The rules that one regular expression each defines, and the rule IDs. A rule set is a
//! bitmask. Pattern rule `i` is the bit `1 << i`, and the length rule is the bit after the
//! last pattern rule.

use regex::{Regex, RegexBuilder};
use std::sync::LazyLock;

/// Each entry is a rule ID and one case-insensitive regular expression. Each expression is
/// the `PATTERNS` entry of the bash checker, unchanged.
const PATTERNS: &[(&str, &str)] = &[];

/// The bit of the length rule.
pub const LENGTH: u16 = 1 << PATTERNS.len();

/// Every rule.
pub const ALL: u16 = (LENGTH << 1) - 1;

static COMPILED: LazyLock<Vec<(&str, Regex)>> = LazyLock::new(|| {
    PATTERNS
        .iter()
        .map(|&(label, pattern)| {
            let regex = RegexBuilder::new(pattern)
                .case_insensitive(true)
                .build()
                .unwrap();
            (label, regex)
        })
        .collect()
});

/// Return each pattern rule ID with its compiled expression, in the order of `PATTERNS`.
pub fn compiled() -> &'static [(&'static str, Regex)] {
    &COMPILED
}

/// Return the bit of the rule with the ID `id`, or `None` for an unknown ID. `all` is not
/// a rule ID.
pub fn rule_bit(id: &str) -> Option<u16> {
    PATTERNS
        .iter()
        .map(|&(label, _)| label)
        .chain(["length"])
        .position(|label| label == id)
        .map(|index| 1 << index)
}

/// Replace each hyphen between two letters or digits with `_`, a word character. `\b` then
/// does not match inside a hyphenated compound: `\bmagic\b` does not match `magic-free`.
/// A finding shows each `_` as `-` again.
pub fn join_compounds(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    chars
        .iter()
        .enumerate()
        .map(|(index, &c)| {
            let joins = c == '-'
                && index > 0
                && chars[index - 1].is_alphanumeric()
                && chars
                    .get(index + 1)
                    .is_some_and(|next| next.is_alphanumeric());
            if joins { '_' } else { c }
        })
        .collect()
}
