use clap::App;

pub struct Config {
    pub show_files: bool,
    pub show_directories: bool,
    pub show_hidden: bool,
    pub show_ignorable: bool,
}

impl Config {

    pub fn new() -> Self {
        let yaml = load_yaml!("../../config/cli.yml");
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
        Config { show_files: show_files, show_directories: show_directories, show_hidden: show_hidden, show_ignorable: show_ignorable }
    }
}
