use std::{ fs, process::{ self, Command } };

use clap::Parser;
use git2::Repository;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(default_value_t = String::from("https://github.com/alexcrichton/git2-rs"))]
    github_repo: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let tempdir = tempfile::tempdir()?;
    println!("{:?}\n", tempdir);

    Repository::clone(&args.github_repo, &tempdir)?;

    let paths = fs::read_dir(&tempdir)?;
    for path in paths {
        println!("{}", path?.path().display());
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg("cargo fmt --check")
        .current_dir(tempdir.as_ref())
        .output()?;

    if output.status.success() {
        println!("Repo is well-formatted according to rustfmt.");
        process::exit(0);
    } else {
        println!("Repo is not well-formatted according to rustfmt.");
        process::exit(1);
    }
}