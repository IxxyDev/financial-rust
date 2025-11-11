use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use parser::Format;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
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

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    let args = Args::parse();

    let input_fmt = Format::from_str(&args.input_format)
        .context("Invalid input format")?;
    let output_fmt = Format::from_str(&args.output_format)
        .context("Invalid output format")?;

    let batch = if args.input == "-" {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin.lock());
        parser::parse(reader, input_fmt)
            .context("Failed to parse from stdin")?
    } else {
        let file = File::open(&args.input)
            .with_context(|| format!("Failed to open input file: {}", args.input))?;
        let reader = BufReader::new(file);
        parser::parse(reader, input_fmt)
            .with_context(|| format!("Failed to parse file: {}", args.input))?
    };

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    parser::write(&batch, &mut writer, output_fmt)
        .context("Failed to write output")?;

    Ok(())
}
