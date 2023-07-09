use std::io;
use urlencoding::decode;

fn rename_file(old_name: &str, new_name: &str) -> std::io::Result<()> {
    println!("renaming \"{}\" to \"{}\"", old_name, new_name);
    std::fs::rename(old_name, new_name)?;
    Ok(())
}

fn main() {
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                let old_name = input.trim();

                match decode(&old_name) {
                    Ok(decoded) => {
                        if old_name == &decoded {
                            input.clear();
                            continue;
                        }
                        match rename_file(&old_name, &decoded) {
                            Ok(_) => {},
                            Err(error) => println!("error renaming file {}\t{}", old_name, error),
                        };
                    },
                    Err(error) => println!("error decoding filename: {}", error),
                }
                input.clear();
            }
            Err(error) => println!("error reading line: {}", error),
        }
    }
}
