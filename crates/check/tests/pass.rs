//! Each fixture in `tests/pass` holds text that the checker MUST NOT report. The checker
//! MUST exit 0 and print nothing.

mod common;

use common::{check, tests_dir};

/// Run the checker on `fixture` in `tests/pass`, and assert that it reports nothing.
fn passes(fixture: &str) {
    let run = check(&tests_dir("pass"), &[fixture]);
    assert!(
        run.status == 0 && run.stdout.is_empty() && run.stderr.is_empty(),
        "{run}"
    );
}

#[test]
fn code_front_matter_fenced_code_and_inline_code_are_not_reported() {
    passes("code.md");
}

#[test]
fn code_a_tilde_fence_is_code_and_a_backtick_fence_line_does_not_close_it() {
    passes("code-tilde.md");
}

#[test]
fn code_a_fence_closes_only_at_a_fence_of_the_same_length_or_longer_without_an_info_string() {
    passes("code-long-fence.md");
}

#[test]
fn code_a_line_indented_by_four_spaces_or_a_tab_after_a_blank_line_is_code() {
    passes("code-indented.md");
}

#[test]
fn code_an_html_comment_on_one_line_or_on_several_lines_is_not_reported() {
    passes("html-comment.md");
}

#[test]
fn source_code_only_the_comments_and_docstrings_of_a_python_file_are_checked() {
    passes("comments.py");
}

#[test]
fn source_code_only_the_comments_of_a_go_file_are_checked() {
    passes("comments.go");
}

#[test]
fn source_code_only_the_comments_of_a_shell_script_are_checked_and_a_path_glob_opens_no_block() {
    passes("comments.sh");
}

#[test]
fn source_code_a_shell_continuation_line_that_starts_with_two_hyphens_is_read_as_a_comment() {
    passes("continuation.sh");
}

#[test]
fn source_code_a_c_preprocessor_directive_is_not_a_comment() {
    passes("preprocessor.c");
}

#[test]
fn source_code_a_marker_in_a_string_literal_or_without_whitespace_before_it_is_no_comment() {
    passes("comments-trailing.py");
}

#[test]
fn length_sentences_within_the_limit_a_bold_lead_in_and_short_list_items_are_not_reported() {
    passes("length.md");
}

#[test]
fn structure_a_heading_a_table_row_and_a_blockquote_that_end_with_a_question_mark_pass() {
    passes("structure.md");
}

#[test]
fn words_a_pattern_inside_a_longer_word_is_not_reported() {
    passes("words.md");
}

#[test]
fn compound_a_pattern_inside_a_hyphenated_compound_is_not_reported() {
    passes("compound.md");
}

#[test]
fn suppress_a_region_without_an_enable_comment_suppresses_every_finding_to_the_end() {
    passes("suppress-region.md");
}

#[test]
fn suppress_a_directive_line_ends_the_paragraph_before_it() {
    passes("suppress-paragraph.md");
}
