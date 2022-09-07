//! Clippy wrapper with very pedantic configuration

mod clippy_flavor;

use std::{
    io,
    process::{Command, Stdio},
};

use clap::Parser;

use clippy_flavor::ClippyFlavor;

const DEVELOPMENT_LINTS: [&str; 19] = [
    "clippy::pedantic",
    "clippy::unwrap_used",
    "clippy::expect_used",
    // "clippy::arithmetic",
    "clippy::integer_arithmetic",
    "clippy::indexing_slicing",
    "clippy::format_push_string",
    "clippy::string_add",
    "clippy::string_add_assign",
    "clippy::string_lit_as_bytes",
    "clippy::string_to_string",
    // "clippy::as_underscore",
    // "clippy::assertions_on_result_states",
    "clippy::clone_on_ref_ptr",
    "clippy::default_union_representation",
    "clippy::rc_buffer",
    "clippy::rc_mutex",
    "clippy::str_to_string",
    "clippy::undocumented_unsafe_blocks",
    "clippy::default_numeric_fallback",
    "clippy::separated_literal_suffix",
    "missing_docs",
];

const PEDANTIC_LINTS: [&str; 1] = ["clippy::todo"];

const DENY_ALWAYS: [&str; 1] = ["non_ascii_idents"];

/// Run a standard set of cargo watch commands and use one of multiple standardised clippy commands
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct CliArgs {
    /// Which clippy flavor to use
    #[clap(value_parser, default_value_t = ClippyFlavor::default())]
    clippy_flavor: ClippyFlavor,

    /// Wether to only warn for lints
    #[clap(short, long, value_parser, default_value_t = false)]
    warn: bool,

    /// Wether to run with release
    #[clap(short = 'O', long, value_parser, default_value_t = false)]
    optimize: bool,
}

fn main() -> io::Result<()> {
    let cli_args = CliArgs::parse();

    let mut cmd = Command::new("cargo");
    // cmd.args(&["+nightly", "clippy"]);
    cmd.arg("clippy");

    if cli_args.optimize {
        cmd.arg("--release");
    }

    cmd.arg("--");

    let action_flag = if cli_args.warn { "-W" } else { "-D" };

    let lints: &[&[&str]] = match cli_args.clippy_flavor {
        ClippyFlavor::Pedantic => &[&DEVELOPMENT_LINTS, &PEDANTIC_LINTS],
        ClippyFlavor::Development => &[&DEVELOPMENT_LINTS],
        ClippyFlavor::Prototype => &[],
    };

    for lint in lints.iter().flat_map(|li| li.iter()) {
        cmd.args(&[action_flag, lint]);
    }

    cmd.args(&[
        "-A",
        "clippy::must_use_candidate",
        "-A",
        "clippy::needless_pass_by_value",
        "-A",
        "clippy::module_name_repetitions",
        "-A",
        "clippy::enum_variant_names",
        action_flag,
        "warnings",
    ]);

    for lint in DENY_ALWAYS {
        cmd.args(&["-D", lint]);
    }

    let output = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if let Some(code) = output.status.code() {
        std::process::exit(code);
    } else {
        Ok(())
    }
}
