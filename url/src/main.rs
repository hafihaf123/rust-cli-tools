use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use urlencoding::{encode, decode};
use std::io::{self, BufRead, IsTerminal, Write};
use std::fs::File;
use std::path::PathBuf;

/// Simple program for basic url encoding and decoding
#[derive(Parser)]
#[command(name = "url")]
#[command(version, about, long_about = None)]
struct Cli {
    /// File to url-encode
    file: Vec<PathBuf>,
    /// Decode mode
    #[arg(short, long)]
    decode: bool,
    /// Save the output to a file
    #[arg(short, long)]
    output: Option<String>,
    /// Url-encode a string instead of a file (cannot be used together with the [FILE] argument)
    #[arg(short, long)]
    string: Option<String>,
}

fn handle_file(path: &PathBuf) -> Result<String> {
    let mut s = std::fs::read_to_string(&path)
        .with_context(|| format!("could not open file: `{}`", path.display()))?;
    s = s.trim().to_string();
    Ok(s)
}

fn handle_stdin() -> Result<String> {
    let stdin = io::stdin();
    if stdin.is_terminal() {
        eprintln!("!! Either the [FILE] or the '-s <STRING>' argument must be specified when not piping input !!\n");
        Cli::command().print_help()?;
        std::process::exit(2);
    }
    let mut res = String::new();

    while stdin.lock().read_line(&mut res)? > 0 {}

    res = res.trim().to_string();
    Ok(res)
}

fn save_res_to_files(res: Vec<String>, output: &str) -> Result<()> {
    let mut new_file = File::create_new(
        PathBuf::from(&output)
    )?;
    for content in res.iter() {
        new_file.write_all(format!("{}\n", content).as_bytes())?;
    }

    Ok(())
}

fn main() -> Result<()> { 
    let cli = Cli::parse();

    let mut string = Vec::new();
    match &cli.string {
        None => {
            if cli.file.is_empty() {
                string.push(handle_stdin()?);
            } else {
                for file_path in cli.file.iter() {
                    string.push(handle_file(file_path)?);
                }
            }
        },
        Some(s) => {
            if cli.file.is_empty() {
                string.push(s.clone());
            }
            else {
                eprintln!("!! cannot use the '-s' option together with the [FILE] argument !!\n");
                Cli::command().print_help()?;
                std::process::exit(2);
            }
        },
    }

    let mut res = Vec::new();
    for s in string {
        if cli.decode {
            res.push(decode(&s)
                .with_context(|| format!("could not decode string: {}", &s))?
                .into_owned());
        } else {
            res.push(encode(&s)
                .into_owned());
        }
    }

    match cli.output {
        Some(output) => {
            save_res_to_files(res, &output)?;
        },
        None => {
            for s in res {
                println!("{}", s);
            }
        },
    }

    Ok(())
}
