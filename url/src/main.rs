use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use urlencoding::{encode, decode};
use std::io::{stdin, IsTerminal};

/// Simple program for basic url encoding and decoding
#[derive(Parser)]
#[command(name = "url")]
#[command(version, about, long_about = None)]
struct Cli{
    /// Decode mode
    #[arg(short, long)]
    decode: bool,
    /// Url-encode a string instead of a file
    #[arg(short, long)]
    string: Option<String>,
    /// File to url-encode
    file: Option<std::path::PathBuf>,
}

fn handle_file(path: std::path::PathBuf) -> Result<String> {
    let mut s = std::fs::read_to_string(&path)
        .with_context(|| format!("could not open file: `{}`", path.display()))?;
    s.pop();
    Ok(s)
}

fn handle_stdin() -> Result<String> {
    if stdin().is_terminal() {
        eprintln!("!! Either the [FILE] or the '-s <STRING>' argument must be specified when not piping input !!\n");
        Cli::command().print_help()?;
        ::std::process::exit(2);
    }
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    s.pop();
    Ok(s)
}

fn main() -> Result<()> { 
    let cli = Cli::parse();

    let string = match cli.string {
        None => {
            match cli.file {
                None => handle_stdin()?,
                Some(path) => handle_file(path)?,
            }
        },
        Some(s) => {
            match cli.file {
                None => s,
                Some(_) => {
                    eprintln!("!! cannot use the '-s' option together with the [FILE] argument !!\n");
                    Cli::command().print_help()?;
                    ::std::process::exit(2);
                }
            }
        },
    };

    let res = if cli.decode {
        decode(&string)
            .with_context(|| format!("could not decode string: {}", string))?
    } else {
        encode(&string)
    };

    println!("{}", res);

    Ok(())
}
