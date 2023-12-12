use std::fs::File;
use std::io::{BufReader, Read};
mod interpreter;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let chars = read_file("test.bf")?;

    println!("{:?}", chars);

    Ok(())
}

fn read_file(path: &str) -> Result<Vec<char>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let chars: Vec<char> = buffer.iter().map(|&byte| byte as char).collect();
    Ok(chars)
}
