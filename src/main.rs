use std::fs;

use clap::Parser;
use git2::Repository;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(default_value_t = String::from("alexcrichton/git2-rs"))]
    github_repo: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let tempdir = tempfile::tempdir()?;
    println!("{:?}\n", tempdir);

    let url = format!("https://github.com/{}", args.github_repo);
    Repository::clone(&url, &tempdir)?;

    let paths = fs::read_dir(&tempdir)?;
    for path in paths {
        println!("{}", path?.path().display());
    }

    Ok(())
}

// Kör rustfmt (finns API?)
// Skriv ut om repot är välformatterat eller ej