mod line_creator;

use line_creator::LineItem;
use std::{fs, io};

fn main() -> io::Result<()> {
    let mut directory_entries: Vec<LineItem> = Vec::new();

    for entry in fs::read_dir(".")? {
        directory_entries.push(LineItem::from_path_buf(entry?.path())?);
    }

    directory_entries.sort();

    for entry in directory_entries {
        println!("{}", &entry);
    }

    Ok(())
}
