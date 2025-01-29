#![forbid(unsafe_code)]

pub mod block;
pub mod chain;
pub mod transaction;
pub mod wallet;

pub use block::*;
pub use chain::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
pub use transaction::*;
pub use wallet::*;

/// Blockchain error.
#[derive(Debug, Error, Serialize, Deserialize, PartialEq)]
pub enum BlockchainError {
    /// Transaction not found.
    #[error("Transaction not found.")]
    TransactionNotFound,

    /// Transaction is invalid.
    #[error("Invalid transaction.")]
    InvalidTransaction,

    /// Invalid configuration during blockchain creation.
    #[error("Invalid configuration during blockchain creation.")]
    InvalidConfiguration,

    /// Insufficient funds.
    #[error("Insufficient funds.")]
    InsufficientFunds,

    /// Wallet not found.
    #[error("Wallet not found.")]
    WalletNotFound,
}
