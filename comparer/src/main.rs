use clap::Parser as ClapParser;
use parser::{Format, Transaction, TransactionBatch};
use std::fs::File;
use std::io::BufReader;
use std::process;
use std::str::FromStr;

#[derive(ClapParser)]
#[command(name = "ypbank_compare")]
#[command(about = "Compare two YPBank transaction files")]
struct Args {
    #[arg(long = "file1", help = "First file path")]
    file1: String,

    #[arg(long = "format1", help = "First file format (csv, text, binary)")]
    format1: String,

    #[arg(long = "file2", help = "Second file path")]
    file2: String,

    #[arg(long = "format2", help = "Second file format (csv, text, binary)")]
    format2: String,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let format1 = Format::from_str(&args.format1)?;
    let format2 = Format::from_str(&args.format2)?;

    let file1 = File::open(&args.file1)?;
    let reader1 = BufReader::new(file1);
    let batch1 = parser::parse(reader1, format1)?;

    let file2 = File::open(&args.file2)?;
    let reader2 = BufReader::new(file2);
    let batch2 = parser::parse(reader2, format2)?;

    compare_batches(&batch1, &batch2, &args.file1, &args.file2)?;

    Ok(())
}

fn compare_batches(
    batch1: &TransactionBatch,
    batch2: &TransactionBatch,
    file1_name: &str,
    file2_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if batch1.transactions.len() != batch2.transactions.len() {
        println!(
            "The files have different number of transactions: {} vs {}",
            batch1.transactions.len(),
            batch2.transactions.len()
        );
        process::exit(1);
    }

    let mut has_differences = false;

    for (i, (tx1, tx2)) in batch1
        .transactions
        .iter()
        .zip(batch2.transactions.iter())
        .enumerate()
    {
        if !transactions_equal(tx1, tx2) {
            if !has_differences {
                println!(
                    "The transaction records in '{}' and '{}' differ:",
                    file1_name, file2_name
                );
                has_differences = true;
            }
            println!("\nTransaction #{} (ID: {}):", i + 1, tx1.id);
            print_transaction_diff(tx1, tx2);
        }
    }

    if !has_differences {
        println!(
            "The transaction records in '{}' and '{}' are identical.",
            file1_name, file2_name
        );
    } else {
        process::exit(1);
    }

    Ok(())
}

fn transactions_equal(tx1: &Transaction, tx2: &Transaction) -> bool {
    tx1.id == tx2.id
        && tx1.posted_at == tx2.posted_at
        && tx1.executed_at == tx2.executed_at
        && tx1.kind == tx2.kind
        && tx1.amount == tx2.amount
        && tx1.description == tx2.description
        && tx1.account == tx2.account
        && tx1.counterparty == tx2.counterparty
        && tx1.category == tx2.category
}

fn print_transaction_diff(tx1: &Transaction, tx2: &Transaction) {
    if tx1.id != tx2.id {
        println!("  ID: '{}' vs '{}'", tx1.id, tx2.id);
    }
    if tx1.posted_at != tx2.posted_at {
        println!("  Posted Date: {} vs {}", tx1.posted_at, tx2.posted_at);
    }
    if tx1.executed_at != tx2.executed_at {
        println!(
            "  Executed Date: {:?} vs {:?}",
            tx1.executed_at, tx2.executed_at
        );
    }
    if tx1.kind != tx2.kind {
        println!("  Kind: {:?} vs {:?}", tx1.kind, tx2.kind);
    }
    if tx1.amount != tx2.amount {
        println!(
            "  Amount: {} {} vs {} {}",
            tx1.amount.amount, tx1.amount.currency, tx2.amount.amount, tx2.amount.currency
        );
    }
    if tx1.description != tx2.description {
        println!(
            "  Description: '{}' vs '{}'",
            tx1.description, tx2.description
        );
    }
    if tx1.account != tx2.account {
        println!("  Account: {:?} vs {:?}", tx1.account, tx2.account);
    }
    if tx1.counterparty != tx2.counterparty {
        println!(
            "  Counterparty: {:?} vs {:?}",
            tx1.counterparty, tx2.counterparty
        );
    }
    if tx1.category != tx2.category {
        println!("  Category: {:?} vs {:?}", tx1.category, tx2.category);
    }
}
