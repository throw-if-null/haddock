//! The command-line entry point. The checker is in the library, so that unit tests call it
//! without a process.

use std::process::ExitCode;

// Go programs call os.Exit. A Rust main returns an ExitCode, so that destructors run and
// buffered output is flushed before the process ends.
fn main() -> ExitCode {
    // args_os accepts an argument that is not valid UTF-8. args panics on it.
    let args: Vec<String> = std::env::args_os()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();
    doc_style_check::run(&args)
}
