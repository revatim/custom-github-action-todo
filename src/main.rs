use std::{env, fs::{read_dir, File}, io::{BufRead, BufReader}, path::Path};

fn main() -> anyhow::Result<()> { 
    // Get Github Workspace path
    // Search project directory for file and subdirectories
    // ? operator is used to propagate errors
    let workspace = env::var("GITHUB_WORKSPACE")?; 
    let count = process_directory(workspace)?;
    println!("Number of TODOs: {count}");
    Ok(())
}
// Recursively process directories
// Accepts any type that can be converted to a Path
fn process_directory(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let path = path.as_ref();
    let entries = read_dir(path)?;
    let mut count = 0;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            count += process_file(path)?;
        } else if path.is_dir() {
            count += process_directory(path)?;
        }
    }
    Ok(count)
}

fn process_file(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let mut count = 0; // mutable variable
    let file = File::open(path.as_ref())?;
    print!("Processing file: {:?}\n", path.as_ref().display());
    let reader = BufReader::new(file); // read complete file 
    for line in reader.lines().filter_map(Result::ok) {
        count += process_line(&line);
    }  
    // let count = content.split_whitespace().count();
    Ok(count)
}

fn process_line(line: &str) -> usize {
    if line.contains("TODO") {
        return 1;
    }
    0
}
// cargo init to create a rust project
mod tests; // import tests module