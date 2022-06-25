use clap::Parser;

mod game;
mod loader;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Height of the map (default is height of terminal)
    #[clap(short, long)]
    height: Option<i32>,

    /// Width of the map (default is width of terminal)
    #[clap(short, long)]
    width: Option<i32>,

    /// Wrap cells around edges (default is no)
    #[clap(short, long)]
    around: bool,

    /// Path to the pattern file
    pattern_path: String,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = game::run_game(args) {
        panic!("{:?}", e);
    }
}
