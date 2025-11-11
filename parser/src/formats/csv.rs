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
    let fields: Vec<&str> = line.split(',').collect();

    if fields.len() < 7 {
        return Err(Error::parse(
            "CSV",
            format!(
                "line {}: insufficient fields (expected at least 7)",
                line_num
            ),
        ));
    }

    let id = fields[0].trim().to_string();

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

    let currency = fields[5].trim().to_string();

    let amount = Money {
        amount: amount_value,
        currency,
    };

    let description = fields[6].trim().to_string();

    let account = if fields.len() > 7 && !fields[7].trim().is_empty() {
        Some(fields[7].trim().to_string())
    } else {
        None
    };

    let counterparty = if fields.len() > 8 && !fields[8].trim().is_empty() {
        Some(fields[8].trim().to_string())
    } else {
        None
    };

    let category = if fields.len() > 9 && !fields[9].trim().is_empty() {
        Some(fields[9].trim().to_string())
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
}
