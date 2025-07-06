use clap::Parser;
use parsers::basic::Aegyorruptor;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    parser: String,

    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    match args.parser.as_str() {
        "mark1" => print!("{}", Aegyorruptor.parse(args.input)),
        _ => return,
    }
}
