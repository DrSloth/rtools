//! Wrapper for a small bu

mod clippy_flavor;

use std::{
    fmt::Write as _,
    io,
    process::{Command, Stdio},
};

use clap::Parser;

use clippy_flavor::ClippyFlavor;

/// Run a standard set of cargo watch commands and use one of multiple standardised clippy commands
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
struct CliArgs {
    /// Which clippy flavor to use
    #[clap(value_parser, default_value_t = ClippyFlavor::default())]
    clippy_flavor: ClippyFlavor,

    /// Wether to also execute `cargo run` at the end
    #[clap(short, long, value_parser, default_value_t = false)]
    run: bool,

    /// Wether rclippy should only warn or deny, deny by default
    #[clap(short, long, value_parser, default_value_t = false)]
    warn: bool,

    /// Rerun on file changes, overrides --optimize
    #[clap(short, long, value_parser, default_value_t = false)]
    observe: bool,

    /// Wether to run this with release (only supported without observe)
    #[clap(short = 'O', long, value_parser, default_value_t = false)]
    optimize: bool,
}

const COMMANDS: [(&str, &str); 5] = [
    ("-x", "check"),
    ("-x", "test"),
    ("-x", "fmt --check"),
    ("-s", "rclippy"),
    ("-x", "run"),
];

fn main() -> io::Result<()> {
    let cli_args = CliArgs::parse();

    // let mut cmd = if cli_args.observe {
    //     observe(cli_args)
    // } else {
    //     check(cli_args)
    // };

    let run_cmd = build_cmd(&cli_args);
    let mut cmd = if cli_args.observe {
        let mut cmd = Command::new("cargo");
        cmd.args(&["watch", "--", "bash", "-c", &run_cmd]);
        cmd
    } else {
        let mut cmd = Command::new("bash");
        cmd.args(&["-c", &run_cmd]);
        cmd
    };

    println!("cmd: {:?}", cmd);

    let output = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if let Some(i) = output.status.code() {
        std::process::exit(i);
    } else {
        Ok(())
    }
}

fn build_cmd(cli_args: &CliArgs) -> String {
    let mut run_arg = String::with_capacity(1024);
    run_arg.push_str("echo 'running rcheck'");
    for (flag, cmd) in COMMANDS {
        match cmd {
            "rclippy" => {
                run_arg.push_str("&&");
                fmt_rclippy(cli_args, &mut run_arg);
            }
            "run" => {
                if cli_args.run {
                    run_arg.push_str("&& cargo run ");
                    if cli_args.optimize {
                        run_arg.push_str("--release");
                    }
                } else {
                    continue;
                }
            }
            s => match flag {
                "-x" => {
                    // let _ = write!(run_arg, "&& cargo {} ", s);
                    run_arg.push_str("&& cargo ");
                    run_arg.push_str(s);
                    if cli_args.optimize {
                        run_arg.push_str("--release");
                    }
                }
                "-s" => {
                    // write!(run_arg, "&& {}", s).unwrap_or_else(|_| panic);
                    run_arg.push_str("&& ");
                    run_arg.push_str(s);
                }
                _ => unreachable!(),
            },
        }
    }

    run_arg
}

// fn observe(cli_args: CliArgs) -> Command {
//     let mut shell_cmd = Command::new("cargo");

//     shell_cmd.arg("watch");
//     for (flag, cmd) in COMMANDS {
//         match cmd {
//             "rclippy" => {
//                 let mut rclippy = String::with_capacity(64);
//                 fmt_rclippy(&cli_args, &mut rclippy);
//                 shell_cmd.args(&[flag, &rclippy]);
//             }
//             "run" => {
//                 if cli_args.run {
//                     shell_cmd.args(&["-x", "run"]);
//                 }
//             }
//             cmd => {
//                 shell_cmd.args(&[flag, cmd]);
//             }
//         }
//     }

//     shell_cmd
// }

// fn check(cli_args: CliArgs) -> Command {
//     // TODO change this to directly run the commands without bash
//     let mut shell_cmd = Command::new("bash");

//     shell_cmd.arg("-c");

//     let mut run_arg = String::with_capacity(1024);
//     run_arg.push_str("echo 'running rcheck'");
//     for (flag, cmd) in COMMANDS {
//         match cmd {
//             "rclippy" => {
//                 run_arg.push_str("&&");
//                 fmt_rclippy(&cli_args, &mut run_arg);
//             }
//             "run" => {
//                 if cli_args.run {
//                     run_arg.push_str("&& cargo run ");
//                     if cli_args.optimize {
//                         run_arg.push_str("--release");
//                     }
//                 } else {
//                     continue;
//                 }
//             }
//             s => match flag {
//                 "-x" => {
//                     let _ = write!(run_arg, "&& cargo {} ", s);
//                     if cli_args.optimize {
//                         run_arg.push_str("--release");
//                     }
//                 }
//                 "-s" => {
//                     let _ = write!(run_arg, "&& {}", s);
//                 }
//                 _ => unreachable!(),
//             },
//         }
//     }

//     println!("{}", run_arg);
//     shell_cmd.arg(run_arg);

//     shell_cmd
// }

fn fmt_rclippy(cli_args: &CliArgs, out: &mut String) {
    let _ = write!(
        out,
        "rclippy {} {} {} ",
        cli_args.clippy_flavor,
        if cli_args.warn { "-w" } else { "" },
        if cli_args.optimize && !cli_args.observe {
            "-O"
        } else {
            ""
        }
    );
}
