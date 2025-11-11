//! Domain models for financial transactions.
//!
//! This module defines the core data structures used to represent
//! financial transactions, including money amounts, transaction types,
//! and batches of transactions.

use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

/// Represents a monetary amount with a specific currency.
///
/// # Examples
///
/// ```
/// use parser::Money;
/// use rust_decimal::Decimal;
///
/// let money = Money {
///     amount: Decimal::new(12345, 2), // 123.45
///     currency: "USD".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Money {
    /// The amount as a decimal value for precise financial calculations
    pub amount: Decimal,
    /// The currency code (e.g., "USD", "EUR", "RUB")
    pub currency: String,
}

/// The type of transaction: incoming (credit) or outgoing (debit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(strum::EnumString, strum::Display)]
pub enum TransactionKind {
    /// Outgoing transaction (withdrawal, payment)
    Debit,
    /// Incoming transaction (deposit, receipt)
    Credit,
}

/// Represents a single financial transaction.
///
/// A transaction includes all relevant information about a financial operation,
/// including dates, amount, parties involved, and categorization.
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    /// Unique identifier for the transaction
    pub id: String,
    /// Date when the transaction was posted to the account
    pub posted_at: NaiveDate,
    /// Optional timestamp when the transaction was actually executed
    pub executed_at: Option<NaiveDateTime>,
    /// Whether this is a debit or credit transaction
    pub kind: TransactionKind,
    /// The monetary amount and currency of the transaction
    pub amount: Money,
    /// Human-readable description of the transaction
    pub description: String,
    /// Optional account identifier
    pub account: Option<String>,
    /// Optional counterparty (the other party in the transaction)
    pub counterparty: Option<String>,
    /// Optional category for transaction classification
    pub category: Option<String>,
}

/// A batch of transactions, optionally associated with an account.
///
/// This structure is used to group multiple transactions together,
/// typically representing all transactions for a specific account
/// or time period.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TransactionBatch {
    /// Optional identifier for the account these transactions belong to
    pub account_id: Option<String>,
    /// The list of transactions in this batch
    pub transactions: Vec<Transaction>,
}
