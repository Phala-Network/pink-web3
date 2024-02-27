# pink-web3

Port of [rust-web3](https://github.com/tomusdrw/rust-web3) to Pink contract.

# web3

Ethereum JSON-RPC multi-transport client.
Rust implementation of Web3.js library.

[![Crates.io](https://img.shields.io/crates/v/web3)](https://crates.io/crates/pink-web3)

[docs-rs-url]: https://docs.rs/pink-web3

Documentation: [crates.io][docs-rs-url]

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
pink-web3 = "0.19.0"
```

## Example
```rust
fn in_some_ink_query() {
    use pink_web3 as web3;

    let transport = web3::transports::PinkHttp::new("http://localhost:3333")?;
    let web3 = web3::Web3::new(transport);

    // Calling accounts
    let mut accounts = web3.eth().accounts().resolve().unwrap();
    accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    for account in accounts {
        let balance = web3.eth().balance(account, None).resolve().unwrap();
        debug_println!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}
```

If you want to deploy smart contracts you have written you can do something like this (make sure you have the solidity compiler installed):

`solc -o build --bin --abi contracts/*.sol`

The solidity compiler is generating the binary and abi code for the smart contracts in a directory called contracts and is being output to a directory called build.

## General
- [ ] More flexible API (accept `Into<X>`)
- [x] Contract calls (ABI encoding; `debris/ethabi`)
- [X] Batch Requests

## Transports
- [x] Pink HTTP transport

## Types
- [x] Types for `U256,H256,Address(H160)`
- [x] Index type (numeric, encoded to hex)
- [x] Transaction type (`Transaction` from Parity)
- [x] Transaction receipt type (`TransactionReceipt` from Parity)
- [x] Block type (`RichBlock` from Parity)
- [x] Work type (`Work` from Parity)
- [X] Syncing type (`SyncStats` from Parity)

## APIs
- [x] Eth: `eth_*`
- [x] Eth filters: `eth_*`
- [x] Eth pubsub: `eth_*`
- [x] `net_*`
- [x] `web3_*`
- [x] `personal_*`
- [ ] `traces_*`

### Parity-specific APIs
- [ ] Parity read-only: `parity_*`
- [ ] Parity accounts: `parity_*` (partially implemented)
- [x] Parity set: `parity_*`
- [ ] `signer_*`

- [x] Own APIs (Extendable)
```rust
let web3 = Web3::new(transport);
web3.api::<CustomNamespace>().custom_method().wait().unwrap()
```

# Cargo Features

The library supports following features:
- `pink` - Enable pink HTTP and (or) signing support
- `signing` - Enable account namespace and local-signing support
- `std` - Enable std features for dependencies
