use std::{fs, io};
use std::fs::{Permissions};
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;

fn create_line(path: PathBuf) -> Result<String,io::Error> {
    let type_specifier = if path.is_dir() { "d" } else { "-" };
    let permissions = create_permission_string(path.metadata()?.permissions());
    let suffix = if path.is_dir() { "/" } else { "" };

    Ok(format!("{}{} {}{}", type_specifier, permissions, path.display(), suffix))
}

fn create_permission_string(permissions: Permissions) -> String {
    let mode = permissions.mode();
    let mut rwx_string = String::new();

    // world
    rwx_string.push(if mode & 0b100000000 > 1  { 'r' } else { '-' });
    rwx_string.push(if mode & 0b010000000 > 1  { 'w' } else { '-' });
    rwx_string.push(if mode & 0b001000000 > 1  { 'x' } else { '-' });
    
    // group
    rwx_string.push(if mode & 0b100000 > 1  { 'r' } else { '-' });
    rwx_string.push(if mode & 0b010000 > 1  { 'w' } else { '-' });
    rwx_string.push(if mode & 0b001000 > 1  { 'x' } else { '-' });
    
    // user
    rwx_string.push(if mode & 0b100 > 0 { 'r' } else { '-' });
    rwx_string.push(if mode & 0b010 > 0 { 'w' } else { '-' });
    rwx_string.push(if mode & 0b001 > 0 { 'x' } else { '-' });

    rwx_string
}

fn main() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        println!("{}", create_line(entry?.path())?);
    }

    Ok(())
}
