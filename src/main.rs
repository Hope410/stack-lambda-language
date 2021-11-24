mod reader;
mod syntax;
use std::path::Path;

fn main() {
    let source = reader::read_file(Path::new("./in/example.sll")).expect("Failed to read file");

    syntax::tree::parse(&source);
}
