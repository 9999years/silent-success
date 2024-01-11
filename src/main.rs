use std::borrow::Cow;
use std::ffi::OsString;
use std::io::Write;
use std::process::Command;

use clap::Parser;
use miette::Context;
use miette::IntoDiagnostic;

/// Runs a program, hiding its output unless it fails.
#[derive(Parser)]
struct Args {
    /// The command to execute.
    command: OsString,
    /// Arguments to pass to the command.
    arguments: Vec<OsString>,
}

impl Args {
    fn display_command(&self) -> Cow<'_, str> {
        self.command.to_string_lossy()
    }
}

fn main() -> miette::Result<()> {
    let args = Args::parse();

    let mut command = Command::new(&args.command);
    command.args(&args.arguments);

    let output = command
        .output()
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to execute {}", args.display_command()))?;

    if !output.status.success() {
        let _ = std::io::stdout().write_all(&output.stdout);
        let _ = std::io::stderr().write_all(&output.stderr);
    }

    Ok(())
}
