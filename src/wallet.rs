use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A wallet that holds a balance of a cryptocurrency.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Wallet {
    /// Unique identifier of the wallet.
    pub id: Uuid,

    /// Unique email address associated with the wallet.
    pub email: String,

    /// Address uniquely identifying the wallet.
    pub address: String,

    /// The current balance of the wallet.
    pub balance: f64,

    /// A history of transactions associated with the wallet.
    pub transaction_hashes: Vec<String>,
}

impl Wallet {
    /// Create a new wallet.
    ///
    /// # Arguments
    ///
    /// - `email`: The email address associated with the wallet.
    /// - `address`: The address uniquely identifying the wallet.
    ///
    /// # Returns
    ///
    /// A new wallet with the given email, address, and balance.
    pub fn new(email: &str, address: &str) -> Self {
        Wallet {
            id: Uuid::new_v4(),
            email: email.to_string(),
            address: address.to_string(),
            balance: 0.0,
            transaction_hashes: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wallet() {
        let email = "email".to_string();
        let address = "0x 1234".to_string();
        let wallet = Wallet::new(&email, &address);

        assert_eq!(wallet.id.get_version(), Some(uuid::Version::Random));
        assert_eq!(wallet.email, email);
        assert_eq!(wallet.address, address);
        assert_eq!(wallet.balance, 0.0);
        assert!(wallet.transaction_hashes.is_empty());
    }
}
