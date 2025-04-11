use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct CliArgs {
    /// The full name of a video capture device (optional)
    #[clap(short, long)]
    pub source: Option<String>,

    /// Display debug output
    #[clap(long)]
    pub debug: bool,
}
