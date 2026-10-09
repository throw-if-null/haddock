//! Invalid inline directives. Each test writes its input to a temporary git repository.

mod common;

use common::Repo;

// The opener is split, so that a check of this file does not read a block here.
const OPEN: &str = concat!("<!", "--");

#[test]
fn an_unknown_rule_in_a_directive_exits_2_and_names_the_rule() {
    let repo = Repo::new();
    repo.write(
        "doc.md",
        &format!("{OPEN} doc-style-disable tnoe -->\nThe idiom is load-bearing.\n"),
    );
    let run = repo.check(&["doc.md"]);
    assert_eq!(run.status, 2, "{run}");
    assert_eq!(run.stderr_text(), "doc.md:1: unknown rule: tnoe\n", "{run}");
    assert!(run.stdout.is_empty(), "{run}");
}

#[test]
fn a_misspelled_directive_exits_2_and_names_the_directive() {
    let repo = Repo::new();
    repo.write("doc.md", &format!("{OPEN} doc-style-disabel tone -->\n"));
    let run = repo.check(&["doc.md"]);
    assert_eq!(run.status, 2, "{run}");
    assert_eq!(
        run.stderr_text(),
        "doc.md:1: unknown directive: doc-style-disabel\n",
        "{run}"
    );
}

#[test]
fn a_directive_without_the_closing_marker_exits_2() {
    let repo = Repo::new();
    repo.write("doc.md", &format!("{OPEN} doc-style-disable tone\n"));
    let run = repo.check(&["doc.md"]);
    assert_eq!(run.status, 2, "{run}");
    assert_eq!(
        run.stderr_text(),
        "doc.md:1: directive does not end with -->\n",
        "{run}"
    );
}

#[test]
fn an_invalid_directive_in_one_file_does_not_stop_the_check_of_the_other_files() {
    let repo = Repo::new();
    repo.write("bad.md", &format!("{OPEN} doc-style-disable tnoe -->\n"));
    repo.write("good.md", "The idiom is load-bearing.\n");
    let run = repo.check(&["bad.md", "good.md"]);
    assert_eq!(run.status, 2, "{run}");
    assert!(
        run.stdout_text()
            .contains("good.md:1: [idiom] load-bearing"),
        "{run}"
    );
}
