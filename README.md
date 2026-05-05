# financial-rust

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/Rust-2021_edition-dea584)

A small Rust workspace for parsing, serializing and comparing financial transaction records across three formats: CSV, plain-text key-value, and a custom binary format with magic number and versioning.

The repo contains three crates: a parser library, a CLI converter, and a CLI comparer.

## Why this exists

I wanted hands-on practice with three Rust topics in one focused project:

- **Format design** — custom binary layout with magic number, version field, and forward-compatible encoding. A serializer that's byte-for-byte stable across runs.
- **Trait-driven I/O** — building everything around `Read` and `Write` so the same code works on files, in-memory buffers, or stdin/stdout without rewriting.
- **Workspace structure** — splitting a small project into a library + multiple CLIs, with shared types and clean Cargo dependencies.

Picking financial transactions as the domain forced me to take serialization seriously: round-trip correctness matters when the data represents money.

## Workspace layout

```
.
├── parser/      # Library: Transaction types, parse() and write() for all formats
├── converter/   # CLI: convert between CSV / text / binary
├── comparer/    # CLI: compare two transaction files for equality
└── examples/    # Sample data in all three formats
```

Common metadata (license, edition, version, repository) lives once in `[workspace.package]` at the workspace root and is inherited by each crate.

## Supported formats

| Format     | Description                                       |
| ---------- | ------------------------------------------------- |
| **CSV**    | Tabular with headers                              |
| **Text**   | Human-readable key-value pairs                    |
| **Binary** | Custom layout with magic number and version field |

Format aliases on the CLI: `csv`, `text` / `txt`, `binary` / `bin`.

## Quick start

```bash
cargo build --release
```

### Convert between formats

```bash
cargo run --release --bin ypbank_converter -- \
  --input examples/transactions.csv \
  --input-format csv \
  --output-format binary > output.bin
```

From stdin:

```bash
cat examples/transactions.csv | cargo run --release --bin ypbank_converter -- \
  --input - \
  --input-format csv \
  --output-format text
```

### Compare files

```bash
cargo run --release --bin ypbank_compare -- \
  --file1 examples/transactions.csv \
  --format1 csv \
  --file2 examples/transactions.bin \
  --format2 binary
```

Output:

```
The transaction records in 'examples/transactions.csv' and 'examples/transactions.bin' are identical.
```

`ypbank_compare` exits with code `1` when the files differ (and prints a per-transaction diff), `0` when they match.

## Library usage

```rust
use parser::{Format, parse, write};
use std::fs::File;
use std::io::BufReader;

let file = File::open("transactions.csv")?;
let reader = BufReader::new(file);
let batch = parse(reader, Format::Csv)?;

let mut output = Vec::new();
write(&batch, &mut output, Format::Binary)?;
```

The library accepts anything implementing `Read` / `Write`, so files, in-memory buffers, and stdin/stdout all work without changes.

## Domain types

- `Transaction` — single transaction record
- `TransactionBatch` — collection of transactions with optional account ID
- `Money` — amount (`rust_decimal::Decimal`) + currency code
- `TransactionKind` — `Credit` / `Debit`

## Error handling

All public functions return `parser::Result<T>` (alias for `Result<T, parser::Error>`):

- `Error::Io` — I/O failures
- `Error::Parse { format, message }` — malformed input, with the format name and detail
- `Error::UnsupportedFormat` — unknown format identifier
- `Error::InvalidFormat` — failure parsing a `Format` enum from a string

## Testing

```bash
cargo test
```

Each format module ships round-trip tests (`test_*_roundtrip`) that write a representative batch and parse it back, asserting field-by-field equality.

## Requirements

- Rust 2021 edition
- Dependencies: `chrono`, `rust_decimal`, `thiserror`, `strum`, `clap` (CLI feature only)

## License

MIT — see [LICENSE](LICENSE).
