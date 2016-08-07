use std::fs;
use std::path::PathBuf;

fn walk_dir(path: PathBuf ) {
    match fs::read_dir(path) {
        Ok(read_dir) => {
            for entry in read_dir {
                match entry {
                    Ok(entry) => {
                        let file_type = entry.file_type().unwrap();
                        println!("{}", entry.path().as_os_str().to_str().unwrap());
                        if file_type.is_dir() {
                            walk_dir(entry.path());
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }
}

fn main() {
    walk_dir(PathBuf::from("."));
}
