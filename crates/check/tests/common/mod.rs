//! Helpers for the integration tests. A test runs the checker as a separate process, as a
//! user does, so the tests also run the bash checker.

// Cargo compiles each file directly in `tests/` as a separate test crate. A file in a
// subdirectory is not a test crate, so each test file includes this module with
// `mod common;`. Go shares the helpers of all `_test.go` files in a package without that
// step. Each test crate uses only part of this module, and the compiler reports the rest as
// unused in that crate.
#![allow(dead_code)]

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The result of one checker run. stdout and stderr stay bytes, because a snippet can end
/// inside a multi-byte character.
pub struct Run {
    pub status: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

// Display gives the text that an assertion message shows: `assert!(..., "{run}")`.
impl fmt::Display for Run {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "exit status {}\n--- stdout\n{}--- stderr\n{}",
            self.status,
            String::from_utf8_lossy(&self.stdout),
            String::from_utf8_lossy(&self.stderr)
        )
    }
}

/// Return the checker under test: the program that the CHECK environment variable names,
/// or the binary of this crate.
pub fn checker() -> PathBuf {
    match std::env::var_os("CHECK") {
        // The tests set the working directory of the checker, so a relative path would
        // then name another file.
        Some(path) => fs::canonicalize(&path)
            .unwrap_or_else(|error| panic!("CHECK={}: {error}", path.display())),
        None => PathBuf::from(env!("CARGO_BIN_EXE_doc-style-check")),
    }
}

/// Return the directory `tests/NAME` of the repository.
pub fn tests_dir(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests")
        .join(name)
}

/// Run the checker in `dir` with `args`.
pub fn check(dir: &Path, args: &[&str]) -> Run {
    let output = Command::new(checker())
        .args(args)
        .current_dir(dir)
        .output()
        .expect("the checker starts");
    Run {
        status: output
            .status
            .code()
            .expect("the checker exits with a status"),
        stdout: output.stdout,
        stderr: output.stderr,
    }
}

/// Assert that `actual` equals the content of the file `expected`, byte for byte. A failure
/// shows the first line that differs, because `assert_eq!` shows a string of several lines
/// on one line.
pub fn assert_snapshot(actual: &[u8], expected: &Path) {
    let wanted =
        fs::read(expected).unwrap_or_else(|error| panic!("{}: {error}", expected.display()));
    if actual == wanted {
        return;
    }
    let actual_lines: Vec<&[u8]> = actual.split(|&byte| byte == b'\n').collect();
    let wanted_lines: Vec<&[u8]> = wanted.split(|&byte| byte == b'\n').collect();
    // When one output is a prefix of the other, the first line after the shorter one
    // differs.
    let index = actual_lines
        .iter()
        .zip(&wanted_lines)
        .position(|(actual, wanted)| actual != wanted)
        .unwrap_or(actual_lines.len().min(wanted_lines.len()));
    let line = |lines: &[&[u8]]| {
        lines.get(index).map_or_else(
            || "(no line)".to_string(),
            |line| String::from_utf8_lossy(line).into_owned(),
        )
    };
    panic!(
        "the output differs from {} at line {}\n  expected: {:?}\n  actual:   {:?}\n--- actual output\n{}",
        expected.display(),
        index + 1,
        line(&wanted_lines),
        line(&actual_lines),
        String::from_utf8_lossy(actual)
    );
}
