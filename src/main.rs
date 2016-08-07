use std::fs;
use std::path::PathBuf;

fn output_file_path(path: PathBuf) {
    let mut cleaned_path = path.as_os_str().to_str().unwrap();
    if path.starts_with("./") {
        let (_, new_path) = cleaned_path.split_at(1 + 1);
        cleaned_path = new_path;
    }
    println!("{}", cleaned_path);
}

fn walk_dir(path: PathBuf) {
    match fs::read_dir(path) {
        Ok(read_dir) => {
            for entry in read_dir {
                match entry {
                    Ok(entry) => {
                        output_file_path(entry.path());
                        let file_type = entry.file_type().unwrap();
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
