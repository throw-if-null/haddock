//! Each fixture in `tests/fail` holds text that the checker reports. The checker MUST exit
//! 1, and its output MUST match the `.expected` file of the fixture byte for byte.

mod common;

use common::{assert_snapshot, check, tests_dir};

/// Run the checker in `tests/fail` with `args`, and compare its output with the file
/// `expected`.
fn fails(args: &[&str], expected: &str) {
    let dir = tests_dir("fail");
    let run = check(&dir, args);
    assert_eq!(run.status, 1, "{run}");
    assert!(run.stderr.is_empty(), "{run}");
    assert_snapshot(&run.stdout, &dir.join(expected));
}

#[test]
fn idiom_every_idiom_in_the_list_is_reported() {
    fails(&["idiom.md"], "idiom.expected");
}

#[test]
fn qualifier_every_vague_qualifier_in_the_list_is_reported() {
    fails(&["qualifier.md"], "qualifier.expected");
}

#[test]
fn filler_every_filler_phrase_in_the_list_is_reported() {
    fails(&["filler.md"], "filler.expected");
}

#[test]
fn anthropomorphism_every_phrase_that_attributes_intent_to_software_is_reported() {
    fails(&["anthropomorphism.md"], "anthropomorphism.expected");
}

#[test]
fn hype_every_promotional_word_in_the_list_is_reported() {
    fails(&["hype.md"], "hype.expected");
}

#[test]
fn tone_exclamations_questions_and_informal_words_are_reported() {
    fails(&["tone.md"], "tone.expected");
}

#[test]
fn chain_an_em_dash_a_double_hyphen_a_semicolon_and_a_trailing_comma_which_are_reported() {
    fails(&["chain.md"], "chain.expected");
}

#[test]
fn length_a_sentence_over_the_limit_is_reported() {
    fails(&["length.md"], "length.expected");
}

#[test]
fn length_max_words_sets_the_limit() {
    fails(
        &["--max-words", "10", "length-max-words.md"],
        "length-max-words.expected",
    );
}

#[test]
fn code_the_text_after_a_code_block_or_an_html_comment_is_reported_on_its_own_line() {
    fails(&["code-end.md"], "code-end.expected");
}

#[test]
fn source_code_findings_in_python_comments_and_docstrings_are_reported() {
    fails(&["comments.py"], "comments.py.expected");
}

#[test]
fn source_code_findings_in_go_line_comments_and_block_comments_are_reported() {
    fails(&["comments.go"], "comments.go.expected");
}

#[test]
fn source_code_a_task_marker_line_in_a_shell_script_is_exempt_from_the_tone_rule_only() {
    fails(&["comments.sh"], "comments.sh.expected");
}

#[test]
fn source_code_findings_in_c_comments_are_reported_and_a_directive_is_not() {
    fails(&["comments.c"], "comments.c.expected");
}

#[test]
fn source_code_a_hash_comment_after_code_is_checked_and_the_string_before_it_is_not() {
    fails(&["comments-trailing.py"], "comments-trailing.py.expected");
}

#[test]
fn source_code_a_double_slash_comment_after_code_is_checked() {
    fails(&["comments-trailing.go"], "comments-trailing.go.expected");
}

#[test]
fn source_code_an_html_comment_block_is_checked() {
    fails(&["comments.html"], "comments.html.expected");
}

#[test]
fn source_code_a_lua_long_comment_is_checked_and_takes_precedence_over_a_line_comment() {
    fails(&["comments.lua"], "comments.lua.expected");
}

#[test]
fn source_code_a_single_quoted_docstring_is_checked() {
    fails(
        &["comments-single-quote.py"],
        "comments-single-quote.py.expected",
    );
}

#[test]
fn source_code_a_line_that_starts_with_two_hyphens_is_a_comment() {
    fails(&["comments.sql"], "comments.sql.expected");
}

#[test]
fn source_code_a_line_that_starts_with_a_semicolon_is_a_comment() {
    fails(&["comments.ini"], "comments.ini.expected");
}

#[test]
fn suppress_a_disable_comment_suppresses_the_listed_rules_until_the_enable_comment() {
    fails(&["suppress-inline.md"], "suppress-inline.expected");
}

#[test]
fn suppress_a_directive_inside_code_is_not_a_directive() {
    fails(&["suppress-code.md"], "suppress-code.expected");
}
