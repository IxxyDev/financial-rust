use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq)]
pub struct Money {
  pub amount: Decimal,
  pub currency: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionKind {
  Debit,
  Credit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
  pub id: String,
  pub posted_at: NaiveDate,
  pub executed_at: Option<NaiveDateTime>,
  pub kind: TransactionKind,
  pub amount: Money,
  pub description: String,
  pub account: Option<String>,
  pub counterparty: Option<String>,
  pub category: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TransactionBatch {
  pub account_id: Option<String>,
  pub transactions: Vec<Transaction>,
}