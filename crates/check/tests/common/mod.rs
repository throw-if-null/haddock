//! Helpers for the integration tests. A test runs the checker as a separate process, as a
//! user does, so the tests also run the bash checker.

// Cargo compiles each file directly in `tests/` as a separate test crate. A file in a
// subdirectory is not a test crate, so each test file includes this module with
// `mod common;`. Go shares the helpers of all `_test.go` files in a package without that
// step. Each test crate uses only part of this module, and the compiler reports the rest as
// unused in that crate.
#![allow(dead_code)]

use std::borrow::Cow;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

/// The result of one checker run. stdout and stderr stay bytes, because a snippet can end
/// inside a multi-byte character.
pub struct Run {
    pub status: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl Run {
    /// Return stdout as text. An invalid byte becomes U+FFFD.
    pub fn stdout_text(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.stdout)
    }

    /// Return stderr as text. An invalid byte becomes U+FFFD.
    pub fn stderr_text(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.stderr)
    }

    /// Report whether the checker exited 0 and printed nothing.
    pub fn reports_nothing(&self) -> bool {
        self.status == 0 && self.stdout.is_empty() && self.stderr.is_empty()
    }
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

/// A git repository in a new temporary directory, for one test. The repository is the
/// directory `repo` with a `.git` directory, and the checker runs in it. A path that a
/// method takes is relative to `repo`, so `..` is the temporary directory.
pub struct Repo {
    // TempDir removes the directory when the value is dropped, as `defer os.RemoveAll(dir)`
    // does in Go.
    temp: TempDir,
}

impl Repo {
    pub fn new() -> Self {
        let repo = Repo {
            temp: TempDir::new().expect("a temporary directory"),
        };
        repo.mkdir(".git");
        repo
    }

    fn path(&self, path: &str) -> PathBuf {
        self.temp.path().join("repo").join(path)
    }

    /// Create the directory at `path` and its parent directories.
    pub fn mkdir(&self, path: &str) {
        let path = self.path(path);
        fs::create_dir_all(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    }

    /// Write `text` to the file at `path`, and create its parent directories.
    pub fn write(&self, path: &str, text: &str) {
        let path = self.path(path);
        let parent = path.parent().expect("the path has a parent directory");
        fs::create_dir_all(parent).unwrap_or_else(|error| panic!("{}: {error}", parent.display()));
        fs::write(&path, text).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    }

    /// Run the checker in `repo` with `args`.
    pub fn check(&self, args: &[&str]) -> Run {
        check(&self.path(""), args)
    }

    /// Run the checker in the directory at `dir` with `args`.
    pub fn check_in(&self, dir: &str, args: &[&str]) -> Run {
        check(&self.path(dir), args)
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
