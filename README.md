# Blockchain

An interface for interacting with a blockchain.

## Reference implementation

[![test](https://github.com/slavik-pastushenko/blockchain-rs/actions/workflows/test.yml/badge.svg)](https://github.com/slavik-pastushenko/blockchain-rs/actions/workflows/test.yml)
[![release](https://github.com/slavik-pastushenko/blockchain-rs/actions/workflows/release.yml/badge.svg?event=workflow_dispatch)](https://github.com/slavik-pastushenko/blockchain-rs/actions/workflows/release.yml)
[![docs](https://docs.rs/blockchain-cli/badge.svg)](https://docs.rs/blockchain-cli)
[![crate](https://img.shields.io/crates/v/blockchain-cli.svg)](https://crates.io/crates/blockchain-cli)
![downloads](https://img.shields.io/crates/d/blockchain-cli)
[![codecov](https://codecov.io/gh/slavik-pastushenko/blockchain-rs/graph/badge.svg?token=9EL0F6725A)](https://codecov.io/gh/slavik-pastushenko/blockchain-rs)

![Features](https://github.com/slavik-pastushenko/blockchain-rs/assets/16807375/28123ed1-aa79-40d7-a59a-3a0710acc381)

## Documentation

For more in-depth details, please refer to the full [documentation](https://docs.rs/blockchain-cli).

If you encounter any issues or have questions that are not addressed in the documentation, feel free to [submit an issue](https://github.com/slavik-pastushenko/blockchain-rs/issues).

## Examples

Explore the capabilities of this blockchain implementation through a set of examples:

- CLI for interacting with the blockchain: [see more](https://github.com/slavik-pastushenko/blockchain-rs/tree/main/examples/cli)
- API for interacting with the blockchain: [see more](https://github.com/slavik-pastushenko/blockchain-rs/tree/main/examples/api)

## Usage

```rust
use blockchain::{ BlockchainError, ChainBuilder };

fn main() -> Result<(), BlockchainError> {
    // Initialise a new blockchain
    let mut chain = match ChainBuilder::default()
        .difficulty(2.0)
        .reward(100.0)
        .fee(0.01)
        .build()
    {
        Ok(chain) => chain,
        Err(e) => return Err(BlockchainError::InvalidConfiguration),
    };

    let sender = chain.create_wallet("sender@mail.com");
    let receiver = chain.create_wallet("receiver@mail.com");

    // Add a transaction
    chain.add_transaction(sender, receiver, 1.25)?;

    // Get a transaction
    let transaction = chain.get_transaction("6e8c5dc01145016e5a979683ba7e13bafaf85e765490aa33c0bba1f41cf581ed")?;

    println!("📦 Transaction: {:?}", transaction);

    Ok(())
}
```

## Features

- `new(difficulty, reward, fee)`: Initialize a new blockchain with the specified parameters.
- `get_transactions(page, size)`: Get a list of current transactions in the blockchain using pagination details.
- `get_transaction(hash)`: Get a transaction by its hash.
- `add_transaction(from, to, amount)`: Add a new transaction to the blockchain.
- `validate_transaction(from, amount)`: Validate a new transaction to the blockchain.
- `create_wallet(email)`: Create a new wallet with a unique email and an initial balance.
- `get_wallet_balance(address)`: Get a wallet's balance based on its address.
- `get_wallet_transactions(address, page, size)`: Get a wallet's transaction history based on its address and using pagination details.
- `get_last_hash()`: Get the hash of the last block in the blockchain.
- `update_difficulty(difficulty)`: Update the mining difficulty of the blockchain.
- `update_reward(reward)`: Update the block reward.
- `update_fee(fee)`: Update the transaction fee.
- `generate_new_block()`: Generate a new block and append it to the blockchain.
- `get_merkle(transactions)`: Calculate the Merkle root hash for a list of transactions.
- `proof_of_work(header)`: Perform the proof-of-work process to mine a block.
- `hash(item)`: Calculate the SHA-256 hash of a serializable item.

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Contributing

Contributions from the community are always welcome! Here are some ways you can contribute:

### Reporting Bugs

If you encounter any bugs, please [submit an issue](https://github.com/slavik-pastushenko/blockchain-rs/issues) with detailed information about the problem and steps to reproduce it.

### Feature Requests

If you have ideas for new features, feel free to [submit an issue](https://github.com/slavik-pastushenko/blockchain-rs/issues) with a detailed description of the feature and its potential use cases.

### Build

To build the project, run:

```bash
cargo build
```

### Test

To run the tests, use:

```bash
cargo test
```

### Lint

Run [clippy](https://github.com/rust-lang/rust-clippy) to lint the code:

```bash
cargo clippy --all-targets --all-features --no-deps -- -D warnings
```

### Format

Run [rustfmt](https://github.com/rust-lang/rustfmt) to format the code:

```bash
cargo fmt
```

### Documentation

Generate documentation in HTML format:

```bash
cargo doc --open
```
