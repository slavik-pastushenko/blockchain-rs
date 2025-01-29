mod common;

use crate::common::setup;

#[test]
fn test_add_transaction() {
    let mut chain = setup();

    let from = chain.create_wallet("s@mail.com");
    let to = chain.create_wallet("r@mail.com");

    let sender = chain.wallets.get_mut(&from).unwrap();
    sender.balance += 20.0;

    let result = chain.add_transaction(from, to, 10.0);

    assert!(result.is_ok());
    assert_eq!(chain.transactions.len(), 1);
}

#[test]
fn test_add_transaction_invalid_balance() {
    let mut chain = setup();

    let from = chain.create_wallet("s@mail.com");
    let to = chain.create_wallet("r@mail.com");

    let sender = chain.wallets.get_mut(&from).unwrap();
    sender.balance += 5.0;

    let result = chain.add_transaction(from, to, 100.0);

    assert!(result.is_err());
}

#[test]
fn test_add_transaction_validation_failed() {
    let mut chain = setup();
    let from = chain.create_wallet("s@mail.com");
    let to = chain.create_wallet("r@mail.com");

    let sender = chain.wallets.get_mut(&from).unwrap();
    sender.balance += 20.0;

    let result = chain.add_transaction(from, to, 0.0);

    assert!(result.is_err());
    assert!(chain.transactions.is_empty());
}

#[test]
fn test_validate_transaction() {
    let mut chain = setup();
    let from = chain.create_wallet("s@mail.com");
    let to = chain.create_wallet("r@mail.com");

    let sender = chain.wallets.get_mut(&from).unwrap();
    sender.balance += 20.0;

    let result = chain.validate_transaction(&from, &to, 10.0);

    assert!(result);
}

#[test]
fn test_validate_transaction_failed_by_invalid_amount() {
    let mut chain = setup();
    let from = chain.create_wallet("s@mail.com");
    let to = chain.create_wallet("r@mail.com");

    let sender = chain.wallets.get_mut(&from).unwrap();
    sender.balance += 20.0;

    let result = chain.validate_transaction(&from, &to, -1.0);

    assert!(!result);
}

#[test]
fn test_validate_transaction_failed_by_invalid_sender() {
    let mut chain = setup();
    let _ = chain.create_wallet("s@mail.com");
    let to = chain.create_wallet("r@mail.com");

    let result = chain.validate_transaction("invalid", &to, 1.0);

    assert!(!result);
}

#[test]
fn test_validate_transaction_failed_by_invalid_receiver() {
    let mut chain = setup();
    let from = chain.create_wallet("s@mail.com");
    let _ = chain.create_wallet("r@mail.com");

    let sender = chain.wallets.get_mut(&from).unwrap();
    sender.balance += 20.0;

    let result = chain.validate_transaction(&from, "invalid", 1.0);

    assert!(!result);
}

#[test]
fn test_validate_transaction_failed_by_same_addresses() {
    let chain = setup();

    let result = chain.validate_transaction("address", "address", 1.0);

    assert!(!result);
}

#[test]
fn test_validate_transaction_failed_by_invalid_sender_balance() {
    let mut chain = setup();
    let from = chain.create_wallet("s@mail.com");
    let to = chain.create_wallet("r@mail.com");

    let result = chain.validate_transaction(&from, &to, 1.0);

    assert!(!result);
}

#[test]
fn test_validate_transaction_failed_by_root() {
    let chain = setup();

    let result = chain.validate_transaction("Root", "Receiver", 0.01);

    assert!(!result);
}

#[test]
fn test_get_transaction() {
    let mut chain = setup();
    let from = chain.create_wallet("s@mail.com");
    let to = chain.create_wallet("r@mail.com");

    let sender = chain.wallets.get_mut(&from).unwrap();
    sender.balance += 20.0;

    chain
        .add_transaction(from.clone(), to.clone(), 10.0)
        .unwrap();

    let transaction = chain
        .get_transaction(&chain.transactions.values().next().unwrap().hash)
        .unwrap();

    assert_eq!(transaction.from, from);
    assert_eq!(transaction.to, to);
}

#[test]
fn test_get_transaction_not_found() {
    let chain = setup();

    let transaction = chain.get_transaction("NonExistentHash");

    assert!(transaction.is_err());
}

#[test]
fn test_get_transactions() {
    let mut chain = setup();
    let from = chain.create_wallet("s@mail.com");
    let to = chain.create_wallet("r@mail.com");

    let sender = chain.wallets.get_mut(&from).unwrap();
    sender.balance += 20.0;

    chain
        .add_transaction(from.clone(), to.clone(), 10.0)
        .unwrap();
    chain
        .add_transaction(to.clone(), from.clone(), 20.0)
        .unwrap();

    let transactions = chain.get_transactions(0, 10);

    assert_eq!(transactions.len(), 2);
}

#[test]
fn test_get_transactions_not_found() {
    let chain = setup();

    let transactions = chain.get_transactions(0, 10);

    assert!(transactions.is_empty());
}

#[test]
fn test_get_transactions_empty_page() {
    let chain = setup();

    let transactions = chain.get_transactions(10, 10);

    assert!(transactions.is_empty());
}

#[test]
fn test_create_wallet() {
    let mut chain = setup();

    let result = chain.create_wallet("s@mail.com");

    assert_eq!(result.len(), 42);
}

#[test]
fn test_get_wallet_balance() {
    let mut chain = setup();
    let address = chain.create_wallet("s@mail.com");

    let result = chain.get_wallet_balance(&address);

    assert!(result.is_some());
}

#[test]
fn test_get_wallet_balance_not_found() {
    let chain = setup();

    let result = chain.get_wallet_balance("address");

    assert!(result.is_none());
}

#[test]
fn test_get_wallet_transactions() {
    let mut chain = setup();

    let from = chain.create_wallet("s@mail.com");
    let to = chain.create_wallet("r@mail.com");

    let sender = chain.wallets.get_mut(&from).unwrap();
    sender.balance += 20.0;

    chain
        .add_transaction(from.clone(), to.clone(), 10.0)
        .unwrap();

    let transactions = chain.get_wallet_transactions(&from, 0, 10).unwrap();

    assert!(!transactions.is_empty());
}

#[test]
fn test_get_new_wallet_transactions() {
    let mut chain = setup();

    let from = chain.create_wallet("s@mail.com");

    let transactions = chain.get_wallet_transactions(&from, 0, 10).unwrap();

    assert!(transactions.is_empty());
}

#[test]
fn test_get_wallet_transactions_not_found() {
    let chain = setup();

    let transactions = chain.get_wallet_transactions("address", 0, 10);

    assert!(transactions.is_none());
}

#[test]
fn test_get_wallet_transactions_empty_page() {
    let chain = setup();

    let transactions = chain.get_wallet_transactions("address", 10, 10);

    assert!(transactions.is_none());
}

#[test]
fn test_get_last_hash() {
    let chain = setup();
    let hash = chain.get_last_hash();

    assert!(!hash.is_empty());
}

#[test]
fn test_update_difficulty() {
    let mut chain = setup();

    chain.update_difficulty(4.0);

    assert_eq!(chain.difficulty, 4.0);
}

#[test]
fn test_update_reward() {
    let mut chain = setup();

    chain.update_reward(50.0);

    assert_eq!(chain.reward, 50.0);
}

#[test]
fn test_update_fee() {
    let mut chain = setup();

    chain.update_fee(0.02);

    assert_eq!(chain.fee, 0.02);
}

#[test]
fn test_generate_new_block() {
    let mut chain = setup();

    let result = chain.generate_new_block();

    assert!(result);
    assert_eq!(chain.chain.len(), 2);
}
