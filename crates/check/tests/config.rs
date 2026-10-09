//! The `.doc-style` project file. Each test builds a git repository in a temporary
//! directory.

mod common;

use common::Repo;

#[test]
fn a_pattern_disables_the_listed_rules_in_the_files_it_matches() {
    let repo = Repo::new();
    repo.write(
        ".doc-style",
        "# Comment lines and blank lines are ignored.\n\ndocs/*.md   tone\n",
    );
    repo.write("docs/a.md", "Is it load-bearing?\n");
    repo.write("README.md", "Is it load-bearing?\n");
    let run = repo.check(&["docs/a.md", "README.md"]);
    let out = run.stdout_text();
    assert_eq!(run.status, 1, "{run}");
    assert!(out.contains("docs/a.md:1: [idiom]"), "{run}");
    assert!(!out.contains("docs/a.md:1: [tone]"), "{run}");
    assert!(out.contains("README.md:1: [tone]"), "{run}");
}

#[test]
fn the_pattern_matches_the_path_relative_to_the_config_file_not_to_the_working_directory() {
    let repo = Repo::new();
    repo.write(".doc-style", "docs/*.md tone\n");
    repo.write("docs/a.md", "Is it done?\n");
    let run = repo.check_in("docs", &["a.md"]);
    assert!(run.reports_nothing(), "{run}");
}

#[test]
fn a_star_in_a_pattern_also_matches_a_slash() {
    let repo = Repo::new();
    repo.write(".doc-style", "docs/* idiom\n");
    repo.write("docs/sub/b.md", "The idiom is load-bearing.\n");
    let run = repo.check(&["docs/sub/b.md"]);
    assert!(run.reports_nothing(), "{run}");
}

#[test]
fn all_disables_every_rule() {
    let repo = Repo::new();
    repo.write(".doc-style", "README.md all\n");
    repo.write("README.md", "Is it load-bearing? It is just robust.\n");
    let run = repo.check(&["README.md"]);
    assert!(run.reports_nothing(), "{run}");
}

#[test]
fn only_the_nearest_config_file_applies() {
    let repo = Repo::new();
    repo.write(".doc-style", "* idiom\n");
    repo.write("docs/.doc-style", "* tone\n");
    repo.write("docs/a.md", "Is it load-bearing?\n");
    let run = repo.check(&["docs/a.md"]);
    let out = run.stdout_text();
    assert_eq!(run.status, 1, "{run}");
    assert!(out.contains("docs/a.md:1: [idiom]"), "{run}");
    assert!(!out.contains("[tone]"), "{run}");
}

#[test]
fn a_config_file_above_the_git_root_does_not_apply() {
    let repo = Repo::new();
    repo.write("../.doc-style", "* idiom\n");
    repo.write("README.md", "The idiom is load-bearing.\n");
    let run = repo.check(&["README.md"]);
    assert_eq!(run.status, 1, "{run}");
    assert!(run.stdout_text().contains("README.md:1: [idiom]"), "{run}");
}

#[test]
fn a_dot_git_file_also_marks_the_git_root() {
    let repo = Repo::new();
    repo.write("../outer/worktree/.git", "");
    repo.write("../outer/.doc-style", "* idiom\n");
    repo.write(
        "../outer/worktree/README.md",
        "The idiom is load-bearing.\n",
    );
    let run = repo.check(&["../outer/worktree/README.md"]);
    assert_eq!(run.status, 1, "{run}");
    assert!(run.stdout_text().contains("[idiom]"), "{run}");
}

#[test]
fn outside_a_git_repository_the_search_continues_upward() {
    let repo = Repo::new();
    repo.write("../plain/.doc-style", "* idiom\n");
    repo.write("../plain/docs/a.md", "The idiom is load-bearing.\n");
    let run = repo.check(&["../plain/docs/a.md"]);
    assert!(run.reports_nothing(), "{run}");
}

#[test]
fn an_inline_enable_comment_does_not_enable_a_rule_that_the_config_file_disables() {
    let repo = Repo::new();
    repo.write(".doc-style", "README.md idiom\n");
    repo.write(
        "README.md",
        "<!-- doc-style-enable idiom -->\nThe idiom is load-bearing.\n",
    );
    let run = repo.check(&["README.md"]);
    assert!(run.reports_nothing(), "{run}");
}

#[test]
fn allow_no_pattern_rule_reports_an_allowed_word_and_other_matches_are_reported() {
    let repo = Repo::new();
    repo.write(
        ".doc-style",
        "# The name of the certificate authority starts with an excluded word.\n\
         allow let's\n\
         allow robust\n",
    );
    repo.write(
        "README.md",
        "The parser is robust. It requests certificates from Let's Encrypt. Let us check.\n",
    );
    let run = repo.check(&["README.md"]);
    let out = run.stdout_text();
    assert_eq!(run.status, 1, "{run}");
    assert!(out.contains("README.md:1: [filler] Let us |"), "{run}");
    assert!(!out.contains("[filler] Let's"), "{run}");
    assert!(!out.contains("[hype]"), "{run}");
}

#[test]
fn allow_the_comparison_ignores_case() {
    let repo = Repo::new();
    repo.write(".doc-style", "allow LET'S\n");
    repo.write(
        "README.md",
        "It requests certificates from Let's Encrypt.\n",
    );
    let run = repo.check(&["README.md"]);
    assert!(run.reports_nothing(), "{run}");
}

#[test]
fn allow_a_word_matches_the_whole_text_of_a_match_not_a_part_of_it() {
    let repo = Repo::new();
    repo.write(".doc-style", "allow magic robustly\n");
    repo.write(
        "README.md",
        "The step works magically. The parser is robust.\n",
    );
    let run = repo.check(&["README.md"]);
    let out = run.stdout_text();
    assert_eq!(run.status, 1, "{run}");
    assert!(
        out.contains("README.md:1: [anthropomorphism] magically |"),
        "{run}"
    );
    assert!(out.contains("README.md:1: [hype] robust |"), "{run}");
}

#[test]
fn allow_without_a_word_exits_2_and_names_the_line() {
    let repo = Repo::new();
    repo.write(".doc-style", "allow\n");
    repo.write("README.md", "Text.\n");
    let run = repo.check(&["README.md"]);
    assert_eq!(run.status, 2, "{run}");
    assert!(
        run.stderr_text()
            .ends_with("/.doc-style:1: no word after allow\n"),
        "{run}"
    );
}

#[test]
fn an_unknown_rule_in_the_config_file_exits_2_and_names_the_line() {
    let repo = Repo::new();
    repo.write(".doc-style", "docs/*.md tone\nREADME.md tnoe\n");
    repo.write("README.md", "Text.\n");
    let run = repo.check(&["README.md"]);
    assert_eq!(run.status, 2, "{run}");
    assert!(
        run.stderr_text()
            .ends_with("/.doc-style:2: unknown rule: tnoe\n"),
        "{run}"
    );
}

#[test]
fn a_pattern_without_a_rule_in_the_config_file_exits_2_and_names_the_line() {
    let repo = Repo::new();
    repo.write(".doc-style", "README.md\n");
    repo.write("README.md", "Text.\n");
    let run = repo.check(&["README.md"]);
    assert_eq!(run.status, 2, "{run}");
    assert!(
        run.stderr_text()
            .ends_with("/.doc-style:1: no rule after pattern: README.md\n"),
        "{run}"
    );
}
