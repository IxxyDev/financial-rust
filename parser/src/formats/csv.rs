use crate::{Error, Money, Result, Transaction, TransactionBatch, TransactionKind};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

/// Parses transaction data from CSV (Comma-Separated Values) format.
///
/// The CSV format expects a header row followed by transaction records.
/// Each line represents one transaction with comma-separated fields.
///
/// # Arguments
///
/// * `reader` - A reader containing CSV transaction data
///
/// # Returns
///
/// Returns a [`TransactionBatch`] with all parsed transactions, or an [`Error`]
/// if the CSV is malformed or contains invalid data.
///
/// # Errors
///
/// This function will return an error if:
/// - The file is empty
/// - The header is invalid
/// - Any line contains invalid data
pub fn parse_csv<R: Read>(reader: R) -> Result<TransactionBatch> {
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();

    let header = lines
        .next()
        .ok_or_else(|| Error::parse("CSV", "empty file"))??;

    if !header.starts_with("TransactionId") {
        return Err(Error::parse("CSV", format!("invalid header: {}", header)));
    }

    let mut transactions = Vec::new();

    for (line_num, line_result) in lines.enumerate() {
        let line = line_result?;

        if line.trim().is_empty() {
            continue;
        }

        let transaction = parse_csv_line(&line, line_num + 2)?;
        transactions.push(transaction);
    }

    Ok(TransactionBatch {
        account_id: None,
        transactions,
    })
}

/// Writes transaction data in CSV (Comma-Separated Values) format.
///
/// This function outputs a header row followed by transaction records,
/// with each field separated by commas. Fields containing special characters
/// are properly escaped.
///
/// # Arguments
///
/// * `batch` - The transaction batch to write
/// * `writer` - A writer to output the CSV data to
///
/// # Returns
///
/// Returns `Ok(())` on success, or an [`Error`] if writing fails.
///
/// # Errors
///
/// This function will return an error if any I/O operation fails.
pub fn write_csv<W: Write>(batch: &TransactionBatch, writer: &mut W) -> Result<()> {
    writeln!(
        writer,
        "TransactionId,PostedDate,ExecutedDate,Type,Amount,Currency,Description,Account,Counterparty,Category"
    )?;

    for transaction in &batch.transactions {
        let executed_date = transaction
            .executed_at
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_default();

        writeln!(
            writer,
            "{},{},{},{},{},{},{},{},{},{}",
            escape_csv_field(&transaction.id),
            transaction.posted_at.format("%Y-%m-%d"),
            executed_date,
            transaction.kind,
            transaction.amount.amount,
            escape_csv_field(&transaction.amount.currency),
            escape_csv_field(&transaction.description),
            escape_csv_field(transaction.account.as_deref().unwrap_or("")),
            escape_csv_field(transaction.counterparty.as_deref().unwrap_or("")),
            escape_csv_field(transaction.category.as_deref().unwrap_or(""))
        )?;
    }

    Ok(())
}

fn parse_csv_line(line: &str, line_num: usize) -> Result<Transaction> {
    let fields = parse_csv_fields(line);

    if fields.len() < 7 {
        return Err(Error::parse(
            "CSV",
            format!(
                "line {}: insufficient fields (expected at least 7)",
                line_num
            ),
        ));
    }

    let id = unescape_csv_field(&fields[0]).trim().to_string();

    let posted_at = NaiveDate::parse_from_str(fields[1].trim(), "%Y-%m-%d").map_err(|e| {
        Error::parse(
            "CSV",
            format!("line {}: invalid posted date: {}", line_num, e),
        )
    })?;

    let executed_at = if fields[2].trim().is_empty() {
        None
    } else {
        Some(
            chrono::NaiveDateTime::parse_from_str(fields[2].trim(), "%Y-%m-%d %H:%M:%S").map_err(
                |e| {
                    Error::parse(
                        "CSV",
                        format!("line {}: invalid executed date: {}", line_num, e),
                    )
                },
            )?,
        )
    };

    let kind = TransactionKind::from_str(fields[3].trim())
        .map_err(|e| Error::parse("CSV", format!("line {}: invalid transaction type: {}", line_num, e)))?;

    let amount_value = Decimal::from_str(fields[4].trim())
        .map_err(|e| Error::parse("CSV", format!("line {}: invalid amount: {}", line_num, e)))?;

    let currency = unescape_csv_field(&fields[5]).trim().to_string();

    let amount = Money {
        amount: amount_value,
        currency,
    };

    let description = unescape_csv_field(&fields[6]).trim().to_string();

    let account = if fields.len() > 7 {
        let val = unescape_csv_field(&fields[7]).trim().to_string();
        if val.is_empty() { None } else { Some(val) }
    } else {
        None
    };

    let counterparty = if fields.len() > 8 {
        let val = unescape_csv_field(&fields[8]).trim().to_string();
        if val.is_empty() { None } else { Some(val) }
    } else {
        None
    };

    let category = if fields.len() > 9 {
        let val = unescape_csv_field(&fields[9]).trim().to_string();
        if val.is_empty() { None } else { Some(val) }
    } else {
        None
    };

    Ok(Transaction {
        id,
        posted_at,
        executed_at,
        kind,
        amount,
        description,
        account,
        counterparty,
        category,
    })
}

/// Parses a CSV line into fields, properly handling quoted fields.
fn parse_csv_fields(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                if in_quotes {
                    // Check if this is an escaped quote (two quotes in a row)
                    if chars.peek() == Some(&'"') {
                        current_field.push('"');
                        chars.next(); // Skip the second quote
                    } else {
                        // End of quoted field
                        in_quotes = false;
                    }
                } else {
                    // Start of quoted field
                    in_quotes = true;
                }
            }
            ',' if !in_quotes => {
                // Field delimiter outside quotes
                fields.push(current_field.clone());
                current_field.clear();
            }
            _ => {
                current_field.push(ch);
            }
        }
    }

    // Push the last field
    fields.push(current_field);
    fields
}

/// Unescapes a CSV field by removing surrounding quotes if present.
fn unescape_csv_field(field: &str) -> String {
    let trimmed = field.trim();
    if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}

/// Escapes a CSV field by quoting it if necessary and escaping internal quotes.
fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_csv_basic() {
        let data = "TransactionId,PostedDate,ExecutedDate,Type,Amount,Currency,Description,Account,Counterparty,Category\n\
                    TX001,2024-01-15,2024-01-15 10:30:00,Credit,1000.50,USD,Salary payment,ACC123,Employer Inc,Salary\n";
        let cursor = Cursor::new(data);
        let batch = parse_csv(cursor).unwrap();

        assert_eq!(batch.transactions.len(), 1);
        assert_eq!(batch.transactions[0].id, "TX001");
        assert_eq!(batch.transactions[0].amount.amount.to_string(), "1000.50");
    }

    #[test]
    fn test_write_csv() {
        let mut batch = TransactionBatch::default();
        batch.transactions.push(Transaction {
            id: "TX001".to_string(),
            posted_at: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            executed_at: None,
            kind: TransactionKind::Credit,
            amount: Money {
                amount: Decimal::from_str("1000.50").unwrap(),
                currency: "USD".to_string(),
            },
            description: "Test transaction".to_string(),
            account: Some("ACC123".to_string()),
            counterparty: None,
            category: None,
        });

        let mut buffer = Vec::new();
        write_csv(&batch, &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("TransactionId"));
        assert!(output.contains("TX001"));
    }

    #[test]
    fn test_csv_roundtrip() {
        // Create test data with special characters
        let original_batch = TransactionBatch {
            account_id: Some("ACC123".to_string()),
            transactions: vec![
                Transaction {
                    id: "TX,001".to_string(), // comma in ID
                    posted_at: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
                    executed_at: Some(
                        chrono::NaiveDateTime::parse_from_str("2024-01-15 10:30:00", "%Y-%m-%d %H:%M:%S")
                            .unwrap(),
                    ),
                    kind: TransactionKind::Credit,
                    amount: Money {
                        amount: Decimal::from_str("1000.50").unwrap(),
                        currency: "USD".to_string(),
                    },
                    description: "Salary \"bonus\" payment".to_string(), // quotes in description
                    account: Some("ACC123".to_string()),
                    counterparty: Some("Employer, Inc".to_string()), // comma in counterparty
                    category: Some("Salary".to_string()),
                },
                Transaction {
                    id: "TX002".to_string(),
                    posted_at: NaiveDate::from_ymd_opt(2024, 1, 16).unwrap(),
                    executed_at: None,
                    kind: TransactionKind::Debit,
                    amount: Money {
                        amount: Decimal::from_str("150.50").unwrap(),
                        currency: "USD".to_string(),
                    },
                    description: "Regular payment".to_string(),
                    account: Some("ACC123".to_string()),
                    counterparty: Some("Store".to_string()),
                    category: Some("Food".to_string()),
                },
            ],
        };

        // Write to CSV
        let mut buffer = Vec::new();
        write_csv(&original_batch, &mut buffer).unwrap();

        // Read back from CSV
        let cursor = Cursor::new(buffer);
        let parsed_batch = parse_csv(cursor).unwrap();

        // Compare
        assert_eq!(parsed_batch.transactions.len(), original_batch.transactions.len());

        for (original, parsed) in original_batch.transactions.iter().zip(parsed_batch.transactions.iter()) {
            assert_eq!(parsed.id, original.id);
            assert_eq!(parsed.posted_at, original.posted_at);
            assert_eq!(parsed.executed_at, original.executed_at);
            assert_eq!(parsed.kind, original.kind);
            assert_eq!(parsed.amount, original.amount);
            assert_eq!(parsed.description, original.description);
            assert_eq!(parsed.account, original.account);
            assert_eq!(parsed.counterparty, original.counterparty);
            assert_eq!(parsed.category, original.category);
        }
    }
}
