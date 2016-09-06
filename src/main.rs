#[macro_use]
extern crate clap;

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use clap::App;

fn walk_dir<F>(path: PathBuf, renderer: Arc<F>) where F : Fn(std::fs::DirEntry) {
    match fs::read_dir(path) {
        Ok(read_dir) => {
            for entry in read_dir {
                match entry {
                    Ok(entry) => {
                        let file_type = entry.file_type().unwrap();
                        let path = entry.path();
                        renderer(entry);
                        if file_type.is_dir() {
                            walk_dir(path, renderer.clone());
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

    let yaml = load_yaml!("../config/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();


    let show_files;
    if matches.is_present("directories_only") {
        show_files = false;
    } else {
        show_files = true;
    }

    let show_directories;
    if matches.is_present("files_only") {
        show_directories = false;
    } else {
        show_directories = true;
    }

    let mut show_hidden = false;
    let mut show_ignorable = false;

    if matches.is_present("all") {
        show_hidden = true;
        show_ignorable = true;
    }

    let path_renderer = |entry: std::fs::DirEntry| {
        let path = entry.path();
        let mut cleaned_path = path.as_os_str().to_str().unwrap();
        if path.starts_with("./") {
            let (_, new_path) = cleaned_path.split_at(2);
            cleaned_path = new_path;
        }

        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() && show_directories {
            println!("{}", cleaned_path);
        }
        if file_type.is_file() && show_files {
            println!("{}", cleaned_path);
        }
    };




    walk_dir(PathBuf::from("."), Arc::new(path_renderer));
}
