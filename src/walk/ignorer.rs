use std::path::PathBuf;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

pub struct Ignorer {
    gitignore_contents: HashMap<String, String>,
}

impl Ignorer {

    pub fn new() -> Self {
        Ignorer { gitignore_contents: HashMap::new() }
    }

    pub fn ignore(&self, path: &PathBuf) -> bool {
        let mut should_ignore = false;
        for (gitignore_path, contents) in &self.gitignore_contents {
            let optional = contents.lines().find(|line| {
                let check_path = gitignore_path.clone() + "/" + line.trim(); 
                check_path == path.to_str().unwrap()
            });
            match optional{
                None => {},
                Some(_) => { 
                    should_ignore = true;
                    break; 
                }
            }
        }
        should_ignore
    }

    pub fn add_gitignore(&mut self, contents: &str, path: &PathBuf) {
        self.gitignore_contents.insert(path.to_str().unwrap().to_string(), contents.to_string());
    }

    pub fn add_ignore_file(&mut self, filepath: &PathBuf) {
        let ignore_file = filepath.clone();
        let mut f = File::open(ignore_file.to_str().unwrap()).unwrap();
        let mut file_contents = String::new();
        f.read_to_string(&mut file_contents).unwrap();
        let ignore_root = ignore_file.parent().unwrap().to_path_buf();
        self.add_gitignore(&file_contents, &ignore_root);
    }
}

#[test]
fn when_a_gitignore_does_NOT_matches_the_filename_it_does_not_get_ignored() {
    let mut ignorer = Ignorer::new();
    let gitignore_contents = "ignore-me.txt";
    let gitignore_path = PathBuf::from("/tmp");
    ignorer.add_gitignore(gitignore_contents, &gitignore_path);
    let file_to_ignore = PathBuf::from("/tmp/dont-ignore-me.txt");
    assert_eq!(ignorer.ignore(&file_to_ignore), false);
}

#[test]
fn when_a_gitignore_matches_the_filename_it_gets_ignored() {
    let mut ignorer = Ignorer::new();
    let gitignore_contents = "ignore-me.txt";
    let gitignore_path = PathBuf::from("/tmp");
    ignorer.add_gitignore(gitignore_contents, &gitignore_path);
    let file_to_ignore = PathBuf::from("/tmp/ignore-me.txt");
    assert_eq!(ignorer.ignore(&file_to_ignore), true);
}

#[test]
fn when_a_gitignore_matches_the_dirname_it_gets_ignored() {
    let mut ignorer = Ignorer::new();
    let gitignore_contents = "ignore-me/";
    let gitignore_path = PathBuf::from("/tmp");
    ignorer.add_gitignore(gitignore_contents, &gitignore_path);
    let file_to_ignore = PathBuf::from("/tmp/ignore-me/");
    assert_eq!(ignorer.ignore(&file_to_ignore), true);
}
