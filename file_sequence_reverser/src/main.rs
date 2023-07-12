use uuid::Uuid;

fn get_dir_contents(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut dir_contents: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();
        dir_contents.push(path_str);
    }
    dir_contents.sort();
    Ok(dir_contents)
}

// swap two file names
fn swap_files(a: &str, b: &str) -> Result<(), std::io::Error> {
    let temp_name = Uuid::new_v4().to_string();
    std::fs::rename(a, &temp_name)?;
    std::fs::rename(b, a)?;
    std::fs::rename(&temp_name, b)?;

    Ok(())
}


// iterate over a list of file paths
fn reverse_file_list(files: Vec<String>) -> Result<(), std::io::Error> {
    let mut a: usize = 0;
    let mut b = files.len() - 1;

    while a < b {
        swap_files(&files[a], &files[b])?;
        a += 1;
        b -= 1;
    }
    Ok(())
}


fn main() {
    let cwd = std::env::current_dir().unwrap();
    let path = cwd.to_str().unwrap().to_string();

    let files = get_dir_contents(&path).unwrap();

    match reverse_file_list(files) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e),
    }
}
