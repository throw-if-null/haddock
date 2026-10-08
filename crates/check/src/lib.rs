//! The doc-style checker. It reports the constructions that the rules in
//! `skills/doc-style/rules.md` exclude. `skills/doc-style/scripts/check` is the reference
//! implementation. The output of this crate matches the output of that script byte for
//! byte, and the bats suite in `tests/` verifies both.

mod config;
mod directive;
mod patterns;
mod strip;

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

use regex::Regex;
use std::sync::LazyLock;

const DEFAULT_MAX_WORDS: u64 = 25;

/// The number of bytes of the source line that a pattern finding shows.
const SNIPPET_BYTES: usize = 52;

const SUMMARY: &str = "Candidates reported. Rewrite each one, or keep it and state the reason.";

// Go has package-level `var task = regexp.MustCompile(...)`, compiled at program start. A
// Rust static must be a constant expression, so LazyLock compiles the regex on first use.
static TASK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[[:space:]]*(TODO|FIXME|XXX|NOTE)\b").unwrap());

struct Options {
    max_words: u64,
    comments: bool,
    files: Vec<String>,
}

/// A command line that the checker rejects. The usage line follows each one.
enum UsageError {
    Usage,
    UnknownOption(String),
}

/// One line of output, with the line number that orders it.
struct Finding {
    line: usize,
    text: Vec<u8>,
}

/// Run the checker with the command-line arguments, `args[0]` included, and return the
/// exit status: 0 no findings, 1 findings, 2 a usage error or a file error.
pub fn run(args: &[String]) -> ExitCode {
    let program = args
        .first()
        .and_then(|arg| arg.rsplit('/').next())
        .unwrap_or("doc-style-check");
    let options = match parse_args(args.get(1..).unwrap_or_default()) {
        Ok(options) => options,
        Err(error) => {
            if let UsageError::UnknownOption(option) = error {
                eprintln!("unknown option: {option}");
            }
            eprintln!("usage: {program} [--max-words N] [--comments] FILE...");
            return ExitCode::from(2);
        }
    };
    match check_files(&options, &mut io::stdout().lock()) {
        Ok(status) => ExitCode::from(status),
        // The output is closed. The bash checker ends on SIGPIPE in this case.
        Err(_) => ExitCode::from(2),
    }
}

fn parse_args(args: &[String]) -> Result<Options, UsageError> {
    let mut options = Options {
        max_words: DEFAULT_MAX_WORDS,
        comments: false,
        files: Vec::new(),
    };
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--max-words" => {
                // `?` returns the error to the caller, as `if err != nil { return err }` in Go.
                let value = args.next().ok_or(UsageError::Usage)?;
                if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
                    return Err(UsageError::Usage);
                }
                // The bash checker accepts any digit string. A value above u64::MAX is a
                // limit that no sentence reaches.
                options.max_words = value.parse().unwrap_or(u64::MAX);
            }
            "--comments" => options.comments = true,
            "-h" | "--help" => return Err(UsageError::Usage),
            _ if arg.starts_with('-') => return Err(UsageError::UnknownOption(arg.clone())),
            _ => options.files.push(arg.clone()),
        }
    }
    if options.files.is_empty() {
        return Err(UsageError::Usage);
    }
    Ok(options)
}

/// Scan every file, write the findings to `out`, and return the exit status. A file with
/// an error is reported on stderr, and the remaining files are still scanned.
fn check_files(options: &Options, out: &mut impl Write) -> io::Result<u8> {
    let mut found = false;
    let mut failed = false;
    for file in &options.files {
        match scan_file(file, options) {
            Ok(findings) => {
                for finding in &findings {
                    out.write_all(&finding.text)?;
                    out.write_all(b"\n")?;
                }
                found |= !findings.is_empty();
            }
            Err(messages) => {
                for message in messages {
                    eprintln!("{message}");
                }
                failed = true;
            }
        }
    }
    if found {
        writeln!(out)?;
        writeln!(out, "{SUMMARY}")?;
    }
    Ok(match (failed, found) {
        (true, _) => 2,
        (false, true) => 1,
        (false, false) => 0,
    })
}

/// Return the findings for one file, in line order. An error is a list of messages, one
/// per invalid line of the `.doc-style` file or per invalid directive.
fn scan_file(file: &str, options: &Options) -> Result<Vec<Finding>, Vec<String>> {
    let path = Path::new(file);
    if !path.is_file() {
        return Err(vec![format!("no such file: {file}")]);
    }
    let config = match config::find(path) {
        Some(config_path) => config::read(&config_path, path)?,
        None => config::Config::default(),
    };
    let bytes = fs::read(path).map_err(|error| vec![format!("cannot read {file}: {error}")])?;
    // The snippet is a prefix of the source line in bytes, so the source lines stay bytes.
    let source: Vec<&[u8]> = bytes.split(|&byte| byte == b'\n').collect();
    let text = String::from_utf8_lossy(&bytes);
    let comments = options.comments || !(file.ends_with(".md") || file.ends_with(".markdown"));
    let mut stripped = if comments {
        strip::comments(&text)
    } else {
        strip::markdown(&text)
    };
    let regions = directive::regions(&stripped, file)?;
    // A directive line matches no pattern and ends the paragraph before it.
    for line in &mut stripped {
        if directive::is_directive(line) {
            line.clear();
        }
    }
    let suppressed = |rule: u16, line: usize| {
        config.off & rule != 0 || regions.get(line - 1).is_some_and(|off| off & rule != 0)
    };

    // The bash checker runs grep once per rule over the whole file. Findings are then
    // sorted by line, so a loop over lines, then rules, gives the same order.
    let mut findings = Vec::new();
    for (line_index, line) in stripped.iter().enumerate() {
        let line_number = line_index + 1;
        let joined = patterns::join_compounds(line);
        for (index, (label, regex)) in patterns::compiled().iter().enumerate() {
            let rule = 1 << index;
            for found in regex.find_iter(&joined) {
                let matched = found.as_str().replace('_', "-");
                if config.allows(&matched)
                    || suppressed(rule, line_number)
                    || (comments && *label == "tone" && exempt(line, &matched))
                {
                    continue;
                }
                let mut text = format!("{file}:{line_number}: [{label}] {matched} | ").into_bytes();
                text.extend_from_slice(snippet(source[line_index]));
                findings.push(Finding {
                    line: line_number,
                    text,
                });
            }
        }
    }
    // sort_by_key is stable, as `sort -s` is: findings on one line keep the rule order.
    findings.sort_by_key(|finding| finding.line);
    Ok(findings)
}

/// Report whether a tone finding in source code is exempt: the comment text starts with a
/// task marker, or the match ends with `?`.
fn exempt(comment: &str, matched: &str) -> bool {
    TASK.is_match(comment) || matched.trim_end_matches(is_space).ends_with('?')
}

/// Return the source line without leading whitespace, cut to SNIPPET_BYTES bytes. The cut
/// can split a multi-byte character, as the bash printf does.
fn snippet(line: &[u8]) -> &[u8] {
    let start = line
        .iter()
        .position(|&byte| !is_space(char::from(byte)))
        .unwrap_or(line.len());
    let line = &line[start..];
    &line[..line.len().min(SNIPPET_BYTES)]
}

/// Report whether `c` is in the POSIX `[[:space:]]` class of the C locale.
/// `char::is_ascii_whitespace` does not include the vertical tab.
fn is_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r')
}

/// Split `text` into lines as awk splits records: at each `\n`, with `\r` kept, and without
/// an empty line after a final `\n`.
fn split_lines(text: &str) -> Vec<&str> {
    if text.is_empty() {
        return Vec::new();
    }
    text.strip_suffix('\n')
        .unwrap_or(text)
        .split('\n')
        .collect()
}
