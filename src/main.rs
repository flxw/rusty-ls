mod line_creator;

use line_creator::LineItem;
use std::{fs, io};
use clap::{AppSettings, Clap, crate_version};

/// This is my personal mimicry version of the ubiquitous `ls' command.
/// I use and extend this application in a personal effort to further my Rust expertise and knowledge.
#[derive(Clap)]
#[clap(version = crate_version!(), author = "Felix W. <felix-registrations@protonmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Set a specific directory to list the arguements in
    #[clap(short, long, default_value = ".")]
    directory: String,

    #[clap(short, long)]
    all: bool
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();

    let mut directory_entries: Vec<LineItem> = Vec::new();

    for entry in fs::read_dir(opts.directory)? {
        let path = entry?.path();
        let is_dotfile = path.as_path().file_name().and_then(|p| p.to_str()).unwrap().starts_with(".");

        if !is_dotfile || (opts.all && is_dotfile) {
            directory_entries.push(LineItem::from_path_buf(path)?);
        }
    }

    directory_entries.sort();

    for entry in directory_entries {
        println!("{}", &entry);
    }

    Ok(())
}
