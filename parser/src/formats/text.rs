use crate::{Error, Money, Result, Transaction, TransactionBatch, TransactionKind};
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

/// Default date used for transactions when posted_at is not yet parsed.
/// This is a valid date (January 1, 1970) that will be replaced during parsing.
/// Using a const ensures compile-time validation of the date.
const DEFAULT_DATE: NaiveDate = match NaiveDate::from_ymd_opt(1970, 1, 1) {
    Some(date) => date,
    None => panic!("Date 01.01.1970 is valid"),
};

/// Parses transaction data from a human-readable plain text format.
///
/// The text format uses key-value pairs separated by colons, with transactions
/// delimited by separator lines ("---"). This format is optimized for human
/// readability and manual editing.
///
/// # Arguments
///
/// * `reader` - A reader containing plain text transaction data
///
/// # Returns
///
/// Returns a [`TransactionBatch`] with all parsed transactions, or an [`Error`]
/// if the text is malformed or contains invalid data.
///
/// # Errors
///
/// This function will return an error if:
/// - The file is empty
/// - Any field contains invalid data
/// - Required fields are missing
pub fn parse_text<R: Read>(reader: R) -> Result<TransactionBatch> {
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();
    let mut transactions = Vec::new();
    let mut account_id = None;

    let header = lines
        .next()
        .ok_or_else(|| Error::parse("Text", "empty file"))??;

    if let Some(acc) = header.strip_prefix("Account: ") {
        account_id = Some(acc.trim().to_string());
    }

    let mut current_transaction: Option<Transaction> = None;

    for (line_num, line_result) in lines.enumerate() {
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            if let Some(tx) = current_transaction.take() {
                transactions.push(tx);
            }
            continue;
        }

        if trimmed == "---" {
            if let Some(tx) = current_transaction.take() {
                transactions.push(tx);
            }
            continue;
        }

        if let Some(id) = trimmed.strip_prefix("ID: ") {
            if let Some(tx) = current_transaction.take() {
                transactions.push(tx);
            }
            current_transaction = Some(Transaction {
                id: id.to_string(),
                posted_at: DEFAULT_DATE,
                executed_at: None,
                kind: TransactionKind::Debit,
                amount: Money {
                    amount: Decimal::ZERO,
                    currency: String::new(),
                },
                description: String::new(),
                account: None,
                counterparty: None,
                category: None,
            });
        } else if let Some(tx) = current_transaction.as_mut() {
            if let Some(date_str) = trimmed.strip_prefix("Date: ") {
                tx.posted_at = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| {
                    Error::parse(
                        "Text",
                        format!("line {}: invalid date: {}", line_num + 2, e),
                    )
                })?;
            } else if let Some(executed_str) = trimmed.strip_prefix("ExecutedDate: ") {
                tx.executed_at = Some(
                    NaiveDateTime::parse_from_str(executed_str, "%Y-%m-%d %H:%M:%S").map_err(
                        |e| {
                            Error::parse(
                                "Text",
                                format!("line {}: invalid executed date: {}", line_num + 2, e),
                            )
                        },
                    )?,
                );
            } else if let Some(kind_str) = trimmed.strip_prefix("Type: ") {
                tx.kind = match kind_str {
                    "Credit" => TransactionKind::Credit,
                    "Debit" => TransactionKind::Debit,
                    other => {
                        return Err(Error::parse(
                            "Text",
                            format!("line {}: invalid type: {}", line_num + 2, other),
                        ))
                    }
                };
            } else if let Some(amount_str) = trimmed.strip_prefix("Amount: ") {
                let parts: Vec<&str> = amount_str.split_whitespace().collect();
                if parts.len() != 2 {
                    return Err(Error::parse(
                        "Text",
                        format!("line {}: invalid amount format", line_num + 2),
                    ));
                }
                tx.amount.amount = Decimal::from_str(parts[0]).map_err(|e| {
                    Error::parse(
                        "Text",
                        format!("line {}: invalid amount: {}", line_num + 2, e),
                    )
                })?;
                tx.amount.currency = parts[1].to_string();
            } else if let Some(desc) = trimmed.strip_prefix("Description: ") {
                tx.description = desc.to_string();
            } else if let Some(acc) = trimmed.strip_prefix("Account: ") {
                tx.account = Some(acc.to_string());
            } else if let Some(counter) = trimmed.strip_prefix("Counterparty: ") {
                tx.counterparty = Some(counter.to_string());
            } else if let Some(cat) = trimmed.strip_prefix("Category: ") {
                tx.category = Some(cat.to_string());
            }
        }
    }

    if let Some(tx) = current_transaction {
        transactions.push(tx);
    }

    Ok(TransactionBatch {
        account_id,
        transactions,
    })
}

/// Writes transaction data in a human-readable plain text format.
///
/// This function outputs transactions using key-value pairs with colons,
/// separated by "---" delimiters. The format is designed to be easy to
/// read and edit manually.
///
/// # Arguments
///
/// * `batch` - The transaction batch to write
/// * `writer` - A writer to output the text data to
///
/// # Returns
///
/// Returns `Ok(())` on success, or an [`Error`] if writing fails.
///
/// # Errors
///
/// This function will return an error if any I/O operation fails.
pub fn write_text<W: Write>(batch: &TransactionBatch, writer: &mut W) -> Result<()> {
    if let Some(account) = &batch.account_id {
        writeln!(writer, "Account: {}", account)?;
        writeln!(writer)?;
    }

    for (i, tx) in batch.transactions.iter().enumerate() {
        if i > 0 {
            writeln!(writer, "---")?;
        }

        writeln!(writer, "ID: {}", tx.id)?;
        writeln!(writer, "Date: {}", tx.posted_at.format("%Y-%m-%d"))?;

        if let Some(executed) = tx.executed_at {
            writeln!(
                writer,
                "ExecutedDate: {}",
                executed.format("%Y-%m-%d %H:%M:%S")
            )?;
        }

        let tx_type = match tx.kind {
            TransactionKind::Credit => "Credit",
            TransactionKind::Debit => "Debit",
        };
        writeln!(writer, "Type: {}", tx_type)?;
        writeln!(
            writer,
            "Amount: {} {}",
            tx.amount.amount, tx.amount.currency
        )?;
        writeln!(writer, "Description: {}", tx.description)?;

        if let Some(acc) = &tx.account {
            writeln!(writer, "Account: {}", acc)?;
        }

        if let Some(counter) = &tx.counterparty {
            writeln!(writer, "Counterparty: {}", counter)?;
        }

        if let Some(cat) = &tx.category {
            writeln!(writer, "Category: {}", cat)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_text_basic() {
        let data = "Account: ACC123\n\n\
                    ID: TX001\n\
                    Date: 2024-01-15\n\
                    Type: Credit\n\
                    Amount: 1000.50 USD\n\
                    Description: Test payment\n\
                    ---\n\
                    ID: TX002\n\
                    Date: 2024-01-16\n\
                    Type: Debit\n\
                    Amount: 50.00 USD\n\
                    Description: Fee\n";

        let cursor = Cursor::new(data);
        let batch = parse_text(cursor).unwrap();

        assert_eq!(batch.account_id.as_deref(), Some("ACC123"));
        assert_eq!(batch.transactions.len(), 2);
        assert_eq!(batch.transactions[0].id, "TX001");
        assert_eq!(batch.transactions[1].id, "TX002");
    }

    #[test]
    fn test_write_text() {
        let mut batch = TransactionBatch {
            account_id: Some("ACC123".to_string()),
            transactions: vec![],
        };

        batch.transactions.push(Transaction {
            id: "TX001".to_string(),
            posted_at: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            executed_at: None,
            kind: TransactionKind::Credit,
            amount: Money {
                amount: Decimal::from_str("1000.50").unwrap(),
                currency: "USD".to_string(),
            },
            description: "Test".to_string(),
            account: None,
            counterparty: None,
            category: None,
        });

        let mut buffer = Vec::new();
        write_text(&batch, &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("ID: TX001"));
        assert!(output.contains("Account: ACC123"));
    }
}
