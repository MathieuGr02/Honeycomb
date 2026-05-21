use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Arguments::parse();
}
