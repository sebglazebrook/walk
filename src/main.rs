extern crate finder;

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::io::prelude::*;
use std::fs::File;

use finder::Config;
use finder::Ignorer;

fn walk_dir<F>(path: PathBuf, entry_callback: Arc<F>, igorablable_filesources: &Vec<&str>, ignorer: &mut Ignorer) where F : Fn(std::fs::DirEntry) {
    // TODO look for ignoreable filesources and add them to ignorer
    match fs::read_dir(path) {
        Ok(read_dir) => {
            for entry in read_dir {
                match entry {
                    Ok(entry) => {

                        let file_name = entry.file_name();
                        let file_name = file_name.to_str().unwrap();
                        let file_type = entry.file_type().unwrap();
                        let path = entry.path();

                        if igorablable_filesources.contains(&file_name) {
                            let mut ignore_file = path.clone();
                            let mut f = File::open(ignore_file.to_str().unwrap()).unwrap();
                            let mut gitignore_contents = String::new();
                            f.read_to_string(&mut gitignore_contents).unwrap();
                            let ignore_root = ignore_file.parent().unwrap().to_path_buf();
                            ignorer.add_gitignore(&gitignore_contents, &ignore_root);
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


    let ignorablable_filesources = vec![".gitignore"];//, ".agignore"];

    let mut ignorer = Ignorer::new();

    walk_dir(PathBuf::from("."), Arc::new(path_renderer), &ignorablable_filesources, &mut ignorer);
}
