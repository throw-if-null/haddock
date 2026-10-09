//! Command-line handling of the checker: usage errors, missing files, and exit codes.

mod common;

use common::{Repo, check, tests_dir};

#[test]
fn no_arguments_exits_2_and_prints_usage() {
    let run = check(&tests_dir("fail"), &[]);
    assert_eq!(run.status, 2, "{run}");
    assert!(run.stderr_text().starts_with("usage:"), "{run}");
}

#[test]
fn an_unknown_option_exits_2_and_names_the_option() {
    let run = check(&tests_dir("fail"), &["--nope", "chain.md"]);
    assert_eq!(run.status, 2, "{run}");
    assert!(
        run.stderr_text().contains("unknown option: --nope"),
        "{run}"
    );
}

#[test]
fn a_non_numeric_max_words_exits_2_and_prints_usage() {
    let run = check(&tests_dir("fail"), &["--max-words", "x", "chain.md"]);
    assert_eq!(run.status, 2, "{run}");
    assert!(run.stderr_text().starts_with("usage:"), "{run}");
}

#[test]
fn comments_reads_a_markdown_file_as_source_code() {
    let repo = Repo::new();
    repo.write(
        "doc.md",
        "# The heading is robust.\n\nThe idiom is load-bearing.\n",
    );
    let run = repo.check(&["--comments", "doc.md"]);
    let out = run.stdout_text();
    assert_eq!(run.status, 1, "{run}");
    assert!(
        out.starts_with("doc.md:1: [hype] robust | # The heading is robust."),
        "{run}"
    );
    assert!(!out.contains("load-bearing"), "{run}");
}

#[test]
fn a_dot_markdown_file_is_read_as_markdown() {
    let repo = Repo::new();
    repo.write("doc.markdown", "The idiom is load-bearing.\n");
    let run = repo.check(&["doc.markdown"]);
    assert_eq!(run.status, 1, "{run}");
    assert!(
        run.stdout_text()
            .starts_with("doc.markdown:1: [idiom] load-bearing"),
        "{run}"
    );
}

#[test]
fn a_missing_file_exits_2_and_names_the_file() {
    let run = check(&tests_dir("fail"), &["missing.md"]);
    assert_eq!(run.status, 2, "{run}");
    assert!(
        run.stderr_text().contains("no such file: missing.md"),
        "{run}"
    );
}

#[test]
fn a_missing_file_with_another_file_exits_2_and_still_reports_the_other_file() {
    let run = check(&tests_dir("fail"), &["missing.md", "chain.md"]);
    assert_eq!(run.status, 2, "{run}");
    assert!(
        run.stderr_text().contains("no such file: missing.md"),
        "{run}"
    );
    assert!(run.stdout_text().contains("chain.md:3: [chain]"), "{run}");
}
