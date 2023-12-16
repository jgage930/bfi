use interpreter::interpreter::Interpreter;

mod interpreter;
mod stack;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    Ok(())
}
