# Reflector Quickstart: A Beginner's Guide to Price Oracles on Stellar

Welcome to the complete guide for integrating Reflector price oracles into your Stellar Soroban smart contracts! This tutorial is designed for beginners and will walk you through everything you need to know.

## 📋 Table of Contents

1. [What is Reflector?](#what-is-reflector)
2. [Architecture Overview](#architecture-overview)
3. [Prerequisites](#prerequisites)
4. [Setting Up Your Environment](#setting-up-your-environment)
5. [Basic Integration](#basic-integration)
6. [Code Examples](#code-examples)
7. [Troubleshooting](#troubleshooting)
8. [Resources](#resources)

## 🤔 What is Reflector?

Reflector is a **decentralized price oracle protocol** for the Stellar network that provides reliable, real-time price data for your smart contracts. Think of it as a bridge between the blockchain and real-world financial data.

### Why Do You Need Price Oracles?

```
Real World Data    →    Blockchain
─────────────────       ──────────
• Stock prices           • Smart contracts
• Crypto prices          • DeFi protocols
• Exchange rates         • Trading bots
• Market data            • Automated systems
```

### Key Features

- ✅ **Real-time price feeds** updated every 5 minutes
- ✅ **SEP-40 compliant** (Stellar standard)
- ✅ **Decentralized consensus** from multiple data sources
- ✅ **Free to use** with no limitations
- ✅ **24-hour data retention** guaranteed

## 🏗️ Architecture Overview

Here's how Reflector works:

```
┌─────────────────┐    ┌──────────────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Your Contract  │    │ Reflector Smartcontract  │    │  Reflector Nodes │    │   Data Sources  │
│                 │    │                          │    │                  │    │                 │
│ • Read prices   │───▶│ • Publish feed           │───▶│ • Aggregate data │───▶│ • CEX APIs      │
│ • Make decisions│    │ • Enforce validation     │    │ • Validate feeds │    │ • DEX protocols │
│ • Execute logic │    │ • Expose interface       │    │ • Reach consensus│    │ • Market APIs   │
└─────────────────┘    └──────────────────────────┘    └──────────────────┘    └─────────────────┘
```

### Data Flow

1. **Collection**: Reflector nodes gather price data from multiple sources
2. **Validation**: Data is validated and normalized
3. **Consensus**: Nodes reach agreement on accurate prices
4. **Storage**: Validated data is stored on-chain in smart contracts
5. **Access**: Your contract reads the price data when needed

## 📋 Prerequisites

Before starting, make sure you have:

- Basic knowledge of Rust programming
- Understanding of blockchain concepts
- Familiarity with Stellar network
- A development environment set up

### Required Tools

- **Rust** (stable toolchain)
- **Soroban CLI**
- **Stellar CLI**

## 🛠️ Setting Up Your Environment

### Step 1: Install Rust

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install stable toolchain with WASM target
rustup install stable
rustup +stable target add wasm32v1-none
```

### Step 2: Install Soroban CLI

```bash
# Install Soroban CLI
cargo install --locked soroban-cli
```

### Step 3: Create Your Project

```bash
# Create a new Soroban project
soroban contract init hello-reflector
cd hello-reflector
```

### Step 4: Add Dependencies

Update your `Cargo.toml`:

```toml
[package]
name = "hello-reflector"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk = "21.0.0"

[dev-dependencies]
soroban-sdk = { version = "21.0.0", features = ["testutils"] }

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true

[profile.release-with-logs]
inherits = "release"
debug-assertions = true
```

## 🚀 Basic Integration

### Understanding SEP-40 Interface

Reflector implements the SEP-40 standard, which defines a common interface for price oracles:

```rust
// SEP-40 Oracle Interface
pub trait PriceOracle {
    // Get the latest price for an asset
    fn lastprice(env: Env, asset: Symbol) -> Option<PriceData>;

    // Get price at specific timestamp
    fn price(env: Env, asset: Symbol, timestamp: u64) -> Option<PriceData>;

    // Get multiple prices in a time range
    fn prices(env: Env, asset: Symbol, records: u32) -> Vec<PriceData>;
}
```

### Price Data Structure

```rust
pub struct PriceData {
    pub price: i128,      // Price with decimal precision
    pub timestamp: u64,   // Unix timestamp
}
```

## 💻 Code Examples

### 1. Save the Reflector interface

First, save the Reflector interface so your contract can call it.

Save as [reflector.rs](./src/reflector.rs)

```rust
// ... code
pub trait Contract {
    fn lastprice(e: Env, asset: Asset) -> Option<PriceData>;
    // ... more code
}
```

### 2. Write your contract

Let's create a new contract in [lib.rs](./src/lib.rs) and import the Reflector traits.

```rust
#![no_std]
use reflector::{Asset as ReflectorAsset, ReflectorClient};
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

mod reflector;

// ... more code coming soon
```

---

Now, save the TESTNET Reflector contract address.

```rust
// ... imports and other things

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(env: Env) {
        let oracle_address = Address::from_str(&env, "CCYOZJCOPG34LLQQ7N24YXBM7LL62R7ONMZ3G6WZAAYPB5OYKOMJRN63");
        env.storage().instance().set(&Symbol::new(&env, "oracle"), &oracle_address);
    }
}
// ... more code coming soon
```

---

Finally, implement the function that interacts with Reflector:

1. Read the oracle address from storage (set in the constructor)
2. Create the Reflector client
3. Instantiate the asset "XLM"
4. Call the Reflector contract
5. Handle the result safely
6. Return the price

```rust
// ... rest of code
pub fn hello_xlm(env: Env) -> i128 {
    let oracle_address: Address = env.storage().instance().get(&Symbol::new(&env, "oracle")).unwrap();
    let reflector_client = ReflectorClient::new(&env, &oracle_address);
    let xlm_asset = ReflectorAsset::Other(Symbol::new(&env, "XLM"));
    let price_data = reflector_client.lastprice(&xlm_asset);

    match price_data {
        Some(data) => data.price,
        None => -1,
    }
}
// DONE!
```

See the full result [here](./src/lib.rs)

### 3. Deploy it

Deploy to testnet using:

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/hello_world.wasm --source alice
```

Remember: you need an account named `alice` with funds to deploy.

### 4. Interact with it

Now that the deploy succeeded, invoke the contract:

```bash
stellar contract invoke --id YOUR_CONTRACT_ADDRESS_HERE --source-account alice --network testnet -- hello_xlm
```

## 🐛 Troubleshooting

### Common Issues

#### 1. "No price data available"

- **Cause**: Asset not supported by oracle or network issues
- **Solution**: Check supported assets list and network connectivity

#### 2. "Price is stale"

- **Cause**: Oracle hasn't updated recently
- **Solution**: Implement fallback mechanisms or increase staleness tolerance

#### 3. "Contract invocation failed"

- **Cause**: Incorrect oracle address or interface mismatch
- **Solution**: Verify oracle contract address and SEP-40 compliance

## 📚 Resources

### Official Documentation

- [Reflector Network](https://reflector.network/)
- [Soroban Documentation](https://soroban.stellar.org/)
- [SEP-40 Standard](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0040.md)

### Oracle Addresses

#### Mainnet

| Feed                   | Contract                                                 | Base symbol   | Decimals | Sampling  | Retention |
| ---------------------- | -------------------------------------------------------- | ------------- | -------- | --------- | --------- |
| External CEX & DEX     | CAFJZQWSED6YAWZU3GWRTOCNPPCGBN32L7QV43XX5LZLFTK6JLN34DLN | USD           | 14       | 5 minutes | 24 hours  |
| Stellar Pubnet         | CALI2BYU2JE6WVRUFYTS6MSBNEHGJ35P4AVCZYF3B6QOE3QKOB2PLE6M | USDCcentre.io | 14       | 5 minutes | 24 hours  |
| Foreign Exchange Rates | CBKGPWGKSKZF52CFHMTRR23TBWTPMRDIYZ4O2P5VS65BMHYH4DXMCJZC | USD           | 14       | 5 minutes | 24 hours  |

#### Testnet

| Feed                   | Contract                                                 | Base symbol   | Decimals | Sampling  | Retention |
| ---------------------- | -------------------------------------------------------- | ------------- | -------- | --------- | --------- |
| External CEX & DEX     | CCYOZJCOPG34LLQQ7N24YXBM7LL62R7ONMZ3G6WZAAYPB5OYKOMJRN63 | USD           | 14       | 5 minutes | 24 hours  |
| Stellar Pubnet         | CAVLP5DH2GJPZMVO7IJY4CVOD5MWEFTJFVPD2YY2FQXOQHRGHK4D6HLP | USDCcentre.io | 14       | 5 minutes | 24 hours  |
| Foreign Exchange Rates | CCSSOHTBL3LEWUCBBEB5NJFC2OKFRC74OWEIJIZLRJBGAAU4VMU5NV4W | USD           | 14       | 5 minutes | 24 hours  |

### Community

- [Reflector Discord](https://discord.gg/v2ggfDty2d)
- [Stellar Developer Discord](https://discord.gg/stellardev)
- [GitHub Repository](https://github.com/stellar/rs-soroban-sdk)

### Example Projects

- [DeFi Lending Protocol](https://github.com/example/defi-lending)
- [Automated Trading Bot](https://github.com/example/trading-bot)
- [Price Alert System](https://github.com/example/price-alerts)

---

## 🎉 Conclusion

You now have a solid foundation for integrating Reflector price oracles into your Soroban smart contracts! Remember to:

- Always validate price freshness
- Implement proper error handling
- Test thoroughly on testnet
- Follow security best practices
- Join the community for support

Happy building! 🚀

```

```
