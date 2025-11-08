use clap::Parser as ClapParser;
use parser::Format;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::process;
use std::str::FromStr;

#[derive(ClapParser)]
#[command(name = "ypbank_converter")]
#[command(about = "Convert YPBank transaction files between different formats")]
struct Args {
    #[arg(short, long, help = "Input file path (use '-' for stdin)")]
    input: String,

    #[arg(long = "input-format", help = "Input format (csv, text, binary)")]
    input_format: String,

    #[arg(long = "output-format", help = "Output format (csv, text, binary)")]
    output_format: String,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input_fmt = Format::from_str(&args.input_format)?;
    let output_fmt = Format::from_str(&args.output_format)?;

    let batch = if args.input == "-" {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin.lock());
        parser::parse(reader, input_fmt)?
    } else {
        let file = File::open(&args.input)?;
        let reader = BufReader::new(file);
        parser::parse(reader, input_fmt)?
    };

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    parser::write(&batch, &mut writer, output_fmt)?;

    Ok(())
}
