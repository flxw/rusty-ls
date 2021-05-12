mod line_creator;
use std::{fs, io};

// next step:
// create struct for containing each entry and sort by type
fn main() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        println!("{}", line_creator::create_line(entry?.path())?);
    }

    Ok(())
}
