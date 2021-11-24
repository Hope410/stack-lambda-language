use std::fs::{File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

// Упрощённая реализация `% cat path`
pub fn read_file(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}