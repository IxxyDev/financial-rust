use crate::{Error, Money, Result, Transaction, TransactionBatch, TransactionKind};
use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;
use std::io::{Read, Write};
use std::str::FromStr;

const MAGIC_NUMBER: u32 = 0x59504246;
const VERSION: u8 = 1;

pub fn parse_binary<R: Read>(mut reader: R) -> Result<TransactionBatch> {
    let magic = read_u32(&mut reader)?;
    if magic != MAGIC_NUMBER {
        return Err(Error::parse("Binary", "invalid magic number"));
    }

    let version = read_u8(&mut reader)?;
    if version != VERSION {
        return Err(Error::parse(
            "Binary",
            format!("unsupported version: {}", version),
        ));
    }

    let account_id = read_optional_string(&mut reader)?;

    let tx_count = read_u32(&mut reader)? as usize;
    let mut transactions = Vec::with_capacity(tx_count);

    for _ in 0..tx_count {
        let transaction = read_transaction(&mut reader)?;
        transactions.push(transaction);
    }

    Ok(TransactionBatch {
        account_id,
        transactions,
    })
}

pub fn write_binary<W: Write>(batch: &TransactionBatch, writer: &mut W) -> Result<()> {
    write_u32(writer, MAGIC_NUMBER)?;
    write_u8(writer, VERSION)?;

    write_optional_string(writer, batch.account_id.as_deref())?;

    write_u32(writer, batch.transactions.len() as u32)?;

    for tx in &batch.transactions {
        write_transaction(writer, tx)?;
    }

    Ok(())
}

fn read_transaction<R: Read>(reader: &mut R) -> Result<Transaction> {
    let id = read_string(reader)?;

    let posted_days = read_u32(reader)?;
    let posted_at = NaiveDate::from_num_days_from_ce_opt(posted_days as i32)
        .ok_or_else(|| Error::parse("Binary", "invalid posted date"))?;

    let has_executed = read_u8(reader)? != 0;
    let executed_at = if has_executed {
        let timestamp = read_i64(reader)?;
        Some(
            chrono::DateTime::from_timestamp(timestamp, 0)
                .ok_or_else(|| Error::parse("Binary", "invalid executed timestamp"))?
                .naive_utc(),
        )
    } else {
        None
    };

    let kind_byte = read_u8(reader)?;
    let kind = match kind_byte {
        0 => TransactionKind::Debit,
        1 => TransactionKind::Credit,
        _ => {
            return Err(Error::parse(
                "Binary",
                format!("invalid kind: {}", kind_byte),
            ))
        }
    };

    let amount_str = read_string(reader)?;
    let amount_value = Decimal::from_str(&amount_str)
        .map_err(|e| Error::parse("Binary", format!("invalid amount: {}", e)))?;

    let currency = read_string(reader)?;

    let amount = Money {
        amount: amount_value,
        currency,
    };

    let description = read_string(reader)?;
    let account = read_optional_string(reader)?;
    let counterparty = read_optional_string(reader)?;
    let category = read_optional_string(reader)?;

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

fn write_transaction<W: Write>(writer: &mut W, tx: &Transaction) -> Result<()> {
    write_string(writer, &tx.id)?;

    let posted_days = tx.posted_at.num_days_from_ce() as u32;
    write_u32(writer, posted_days)?;

    if let Some(executed) = tx.executed_at {
        write_u8(writer, 1)?;
        let timestamp = executed.and_utc().timestamp();
        write_i64(writer, timestamp)?;
    } else {
        write_u8(writer, 0)?;
    }

    let kind_byte = match tx.kind {
        TransactionKind::Debit => 0,
        TransactionKind::Credit => 1,
    };
    write_u8(writer, kind_byte)?;

    write_string(writer, &tx.amount.amount.to_string())?;
    write_string(writer, &tx.amount.currency)?;
    write_string(writer, &tx.description)?;

    write_optional_string(writer, tx.account.as_deref())?;
    write_optional_string(writer, tx.counterparty.as_deref())?;
    write_optional_string(writer, tx.category.as_deref())?;

    Ok(())
}

fn read_u8<R: Read>(reader: &mut R) -> Result<u8> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;
    Ok(buf[0])
}

fn write_u8<W: Write>(writer: &mut W, value: u8) -> Result<()> {
    writer.write_all(&[value])?;
    Ok(())
}

fn read_u32<R: Read>(reader: &mut R) -> Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn write_u32<W: Write>(writer: &mut W, value: u32) -> Result<()> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

fn read_i64<R: Read>(reader: &mut R) -> Result<i64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(i64::from_le_bytes(buf))
}

fn write_i64<W: Write>(writer: &mut W, value: i64) -> Result<()> {
    writer.write_all(&value.to_le_bytes())?;
    Ok(())
}

fn read_string<R: Read>(reader: &mut R) -> Result<String> {
    let len = read_u32(reader)? as usize;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|e| Error::parse("Binary", format!("invalid UTF-8: {}", e)))
}

fn write_string<W: Write>(writer: &mut W, s: &str) -> Result<()> {
    write_u32(writer, s.len() as u32)?;
    writer.write_all(s.as_bytes())?;
    Ok(())
}

fn read_optional_string<R: Read>(reader: &mut R) -> Result<Option<String>> {
    let has_value = read_u8(reader)? != 0;
    if has_value {
        Ok(Some(read_string(reader)?))
    } else {
        Ok(None)
    }
}

fn write_optional_string<W: Write>(writer: &mut W, s: Option<&str>) -> Result<()> {
    if let Some(value) = s {
        write_u8(writer, 1)?;
        write_string(writer, value)?;
    } else {
        write_u8(writer, 0)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_and_write_binary() {
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
            account: Some("ACC123".to_string()),
            counterparty: None,
            category: Some("Salary".to_string()),
        });

        let mut buffer = Vec::new();
        write_binary(&batch, &mut buffer).unwrap();

        let cursor = Cursor::new(buffer);
        let parsed = parse_binary(cursor).unwrap();

        assert_eq!(parsed.account_id, batch.account_id);
        assert_eq!(parsed.transactions.len(), 1);
        assert_eq!(parsed.transactions[0].id, "TX001");
        assert_eq!(parsed.transactions[0].amount.amount.to_string(), "1000.50");
    }

    #[test]
    fn test_invalid_magic_number() {
        let data = vec![0, 0, 0, 0];
        let cursor = Cursor::new(data);
        let result = parse_binary(cursor);
        assert!(result.is_err());
    }
}
