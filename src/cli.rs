use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "A brain f**k interpreter written in rust."
)]
pub struct Cli {
    /// Path to the .bf file to execute.
    #[arg(short, long)]
    pub path: String,
}
