//! Checks of the test suite itself. Cargo does not discover fixture files, so a fixture
//! without a test is never run. These tests detect that.

mod common;

use std::fs;

use common::tests_dir;

/// Return the names of the fixtures in `tests/DIR`, without the `.expected` files.
fn fixtures(dir: &str) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(tests_dir(dir))
        .expect("the fixture directory exists")
        .map(|entry| entry.expect("the entry is readable").file_name())
        .map(|name| name.into_string().expect("the name is UTF-8"))
        .filter(|name| !name.ends_with(".expected"))
        .collect();
    names.sort();
    names
}

#[test]
fn every_fixture_is_run_by_a_test() {
    // include_str! embeds a file in the test binary at compile time, as `//go:embed` does in
    // Go.
    let sources = [
        ("fail", include_str!("fail.rs")),
        ("pass", include_str!("pass.rs")),
    ];
    let mut missing = Vec::new();
    for (dir, source) in sources {
        for fixture in fixtures(dir) {
            if !source.contains(&format!("\"{fixture}\"")) {
                missing.push(format!("{dir}/{fixture}"));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "no test runs these fixtures: {missing:?}"
    );
}

#[test]
fn every_fail_fixture_states_its_rule_on_line_1() {
    let mut missing = Vec::new();
    for fixture in fixtures("fail") {
        let text = fs::read_to_string(tests_dir("fail").join(&fixture))
            .expect("the fixture is readable UTF-8");
        let first = text.lines().next().unwrap_or_default();
        // The opener is split, so that a check of this file does not read a block here.
        let rule = first
            .strip_prefix(concat!("<!", "-- Rule: "))
            .and_then(|rest| rest.strip_suffix(" -->"));
        if rule.is_none() {
            missing.push(fixture);
        }
    }
    assert!(
        missing.is_empty(),
        "line 1 is not a Rule comment: {missing:?}"
    );
}
