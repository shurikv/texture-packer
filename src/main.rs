mod algorithm;
mod sprite;
mod export;

use clap::Parser;
use clap::Subcommand;

#[derive(
    clap::ValueEnum, Clone, Default, Debug
)]
enum OutputFormat {
    #[default]
    Xml,
    Json,
    UnityJson
}

#[derive(
clap::ValueEnum, Clone, Default, Debug
)]
enum Algorithm {
    #[default]
    Grid
}

#[derive(Parser,Debug)]
#[clap(author, version, about="CLI tools for texture packing")]
struct Cli {
    #[clap(short, long)]
    input_folder: String,
    #[clap(short, long)]
    output_folder: String,
    #[clap(value_enum)]
    #[clap(short= 'f', long)]
    format: OutputFormat,
    #[clap(value_enum)]
    #[clap(short, long)]
    algorithm: Algorithm
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    println!("args: {:?}", args);
}
