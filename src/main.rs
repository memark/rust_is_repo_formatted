fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir().unwrap();
    println!("{:?}\n", tempdir);

    let url = "https://github.com/alexcrichton/git2-rs";
    let _repo = git2::Repository::clone(url, &tempdir)?;

    let paths = std::fs::read_dir(&tempdir)?;
    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }

    Ok(())
}