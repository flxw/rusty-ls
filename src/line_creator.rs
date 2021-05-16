use std::io;
use std::fs::{Permissions};
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use std::fmt;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
pub enum LineItemType {
    Directory,
    File
}

pub struct LineItem {
    item_type: LineItemType,
    permissions_string: String,
    name: String
}

impl LineItem {
    pub fn new(item_type: LineItemType, permissions_string: String, name: String) -> LineItem {
        LineItem {
            item_type,
            permissions_string,
            name
        }
    }

    pub fn from_path_buf(path: PathBuf) -> Result<LineItem, io::Error> {
        let item_type = if path.is_dir() { LineItemType::Directory } else { LineItemType::File };
        let permissions = LineItem::create_permission_string(path.metadata()?.permissions());
        let name = path.as_path().file_name().and_then(|p| Some(p.to_string_lossy())).unwrap();

        Ok(LineItem::new(item_type, permissions, name.into_owned()))
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
}

impl fmt::Display for LineItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.item_type {
            LineItemType::Directory => write!(f, "d{} {}/", self.permissions_string, self.name),
            _ => write!(f, "-{} {}", self.permissions_string, self.name)
        }
    }
}

impl Ord for LineItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match &self.item_type {
            LineItemType::Directory => match &other.item_type {
                LineItemType::Directory => self.name.cmp(&other.name),
                LineItemType::File => Ordering::Less
            },

            LineItemType::File => match &other.item_type {
                LineItemType::File => self.name.cmp(&other.name),
                LineItemType::Directory => Ordering::Greater
            }
        }
    }
}

impl PartialOrd for LineItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LineItem {
    fn eq(&self, other: &Self) -> bool {
        self.item_type == other.item_type &&
        self.name == self.name &&
        self.permissions_string == self.permissions_string
    }
}

impl Eq for LineItem {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_file_item_from_path_buf() {
        let path_buf = PathBuf::from("Cargo.toml");
        let item = LineItem::from_path_buf(path_buf).expect("");
        
        assert_eq!(item.item_type, LineItemType::File);
        assert_eq!(item.name, "Cargo.toml");
    }

    #[test]
    fn constructs_dir_item_from_path_buf() {
        let path_buf = PathBuf::from(".git");
        let item = LineItem::from_path_buf(path_buf).expect("");
        
        assert_eq!(item.item_type, LineItemType::Directory);
        assert_eq!(item.name, ".git");
    }
}