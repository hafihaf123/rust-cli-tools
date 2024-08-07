use anyhow::{Context, Result, anyhow};
use clap::{CommandFactory, Parser};
use urlencoding::{encode, decode};
use std::io::{stdin, IsTerminal, Write};
use std::fs::File;
use std::path::PathBuf;

/// Simple program for basic url encoding and decoding
#[derive(Parser)]
#[command(name = "url")]
#[command(version, about, long_about = None)]
struct Cli {
    /// File to url-encode. The output will be saved to <filename>_encoded.txt or <filename>_decoded.txt
    file: Vec<PathBuf>,
    /// Decode mode
    #[arg(short, long)]
    decode: bool,
    /// Url-encode a string instead of a file (cannot be used together with the [FILE] argument). Output will be written to stdout.
    #[arg(short, long)]
    string: Option<String>,
}

fn handle_file(path: &PathBuf) -> Result<String> {
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

fn save_res_to_files(res: Vec<String>, files: Vec<PathBuf>, is_decode: bool) -> Result<()> {
    if res.len() != files.len() {
        return Err(anyhow!("The results vector and the files vector don't have the same length"));
    }

    for (file_path, content) in files.iter().zip(res.iter()) {
        let mut new_file = if is_decode {
            File::create_new(
                PathBuf::from(
                    format!("{}_decoded.txt", file_path.display())
                )
            )?
        } else {
            File::create_new(
                PathBuf::from(
                    format!("{}_encoded.txt", file_path.display())
                )
            )?
        };

        new_file.write_all(content.as_bytes())?;
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
                ::std::process::exit(2);
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

    if cli.string.is_none() {
        save_res_to_files(res, cli.file, cli.decode)?;
    } else {
        println!("{}", res[0]);
    }

    Ok(())
}
