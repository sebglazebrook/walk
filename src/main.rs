extern crate walk;

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use walk::Config;
use walk::Ignorer;

fn walk_dir<F>(path: PathBuf, entry_callback: Arc<F>, igorablable_filesources: &Vec<&str>, ignorer: &mut Ignorer) where F : Fn(std::fs::DirEntry) {
    match fs::read_dir(path) {
        Ok(read_dir) => {
            for entry in read_dir {
                match entry {
                    Ok(entry) => {

                        let temp_file_name = entry.file_name();
                        let file_name = temp_file_name.to_str().unwrap();

                        let file_type = entry.file_type().unwrap();
                        let path = entry.path();

                        if igorablable_filesources.contains(&file_name) {
                            ignorer.add_ignore_file(&path);
                        }

                        if !ignorer.ignore(&path) {
                            entry_callback(entry);
                            if file_type.is_dir() {
                                walk_dir(path, entry_callback.clone(), igorablable_filesources, ignorer);
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }
}

fn generate_path_from_entry(entry: &std::fs::DirEntry) -> String {
    let path = entry.path();
    let mut cleaned_path = path.as_os_str().to_str().unwrap();
    if path.starts_with("./") {
        let (_, new_path) = cleaned_path.split_at(2);
        cleaned_path = new_path;
    }
    cleaned_path.to_string()
}


fn main() {

    let config = Config::new();

    let path_renderer = |entry: std::fs::DirEntry| {
        let path = generate_path_from_entry(&entry);

        if path.starts_with(".") && !config.show_hidden { return; }

        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() && config.show_directories { println!("{}", path); }
        if file_type.is_file() && config.show_files { println!("{}", path); }
    };

    let mut ignorablable_filesources = Vec::new();
    let mut ignorer = Ignorer::new();
    if !config.show_ignorable {
      ignorablable_filesources.push(".gitignore");
      ignorablable_filesources.push(".agignore");
    }

    walk_dir(config.start_directory.clone(), Arc::new(path_renderer), &ignorablable_filesources, &mut ignorer);
}
