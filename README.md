# Reflector Protocol Tutorial: A Beginner's Guide to Price Oracles on Stellar

Welcome to the complete guide for integrating Reflector price oracles into your Stellar Soroban smart contracts! This tutorial is designed for beginners and will walk you through everything you need to know.

## 📋 Table of Contents

1. [What is Reflector?](#what-is-reflector)
2. [Architecture Overview](#architecture-overview)
3. [Prerequisites](#prerequisites)
4. [Setting Up Your Environment](#setting-up-your-environment)
5. [Basic Integration](#basic-integration)
6. [Code Examples](#code-examples)
7. [Advanced Usage](#advanced-usage)
8. [Best Practices](#best-practices)
9. [Troubleshooting](#troubleshooting)
10. [Resources](#resources)

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
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Data Sources  │    │  Reflector Nodes │    │ Your Contract   │
│                 │    │                  │    │                 │
│ • CEX APIs      │───▶│ • Aggregate data │───▶│ • Read prices   │
│ • DEX protocols │    │ • Validate feeds │    │ • Make decisions│
│ • Market APIs   │    │ • Reach consensus│    │ • Execute logic │
└─────────────────┘    └──────────────────┘    └─────────────────┘
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
- **Code editor** (VS Code recommended)

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

### Example 1: Basic Price Reader Contract

Let's create a simple contract that reads prices from Reflector:

```rust
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, symbol_short};

// Define the Reflector oracle interface
#[contractimpl]
pub trait ReflectorOracle {
    fn lastprice(env: Env, asset: Symbol) -> Option<(i128, u64)>;
}

#[contract]
pub struct PriceReader;

#[contractimpl]
impl PriceReader {
    /// Get the latest price for an asset from Reflector
    pub fn get_price(
        env: Env, 
        oracle_address: Address, 
        asset: Symbol
    ) -> Option<i128> {
        // Create client for the Reflector oracle contract
        let oracle = ReflectorOracleClient::new(&env, &oracle_address);
        
        // Get the latest price
        if let Some((price, timestamp)) = oracle.lastprice(&asset) {
            // Check if price is not stale (within 10 minutes)
            let current_time = env.ledger().timestamp();
            if current_time - timestamp <= 600 {
                Some(price)
            } else {
                None // Price is stale
            }
        } else {
            None // No price available
        }
    }
}
```

### Example 2: Price Comparison Contract

```rust
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, symbol_short};

#[contract]
pub struct PriceComparator;

#[contractimpl]
impl PriceComparator {
    /// Compare prices of two assets
    pub fn compare_prices(
        env: Env,
        oracle_address: Address,
        asset_a: Symbol,
        asset_b: Symbol,
    ) -> i32 {
        let oracle = ReflectorOracleClient::new(&env, &oracle_address);
        
        let price_a = oracle.lastprice(&asset_a);
        let price_b = oracle.lastprice(&asset_b);
        
        match (price_a, price_b) {
            (Some((price_a, _)), Some((price_b, _))) => {
                if price_a > price_b {
                    1   // Asset A is more expensive
                } else if price_a < price_b {
                    -1  // Asset B is more expensive
                } else {
                    0   // Prices are equal
                }
            }
            _ => panic!("Unable to get prices for comparison"),
        }
    }
}
```

### Example 3: DeFi Liquidation Contract

```rust
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec};

#[contract]
pub struct LiquidationBot;

#[contractimpl]
impl LiquidationBot {
    /// Check if a position should be liquidated
    pub fn should_liquidate(
        env: Env,
        oracle_address: Address,
        collateral_asset: Symbol,
        debt_asset: Symbol,
        collateral_amount: i128,
        debt_amount: i128,
        liquidation_ratio: i128, // e.g., 150% = 1500000 (with 6 decimals)
    ) -> bool {
        let oracle = ReflectorOracleClient::new(&env, &oracle_address);
        
        // Get current prices
        let collateral_price = oracle.lastprice(&collateral_asset);
        let debt_price = oracle.lastprice(&debt_asset);
        
        match (collateral_price, debt_price) {
            (Some((col_price, _)), Some((debt_price, _))) => {
                // Calculate collateral value in debt asset terms
                let collateral_value = (collateral_amount * col_price) / debt_price;
                
                // Calculate current ratio (with 6 decimal precision)
                let current_ratio = (collateral_value * 1_000_000) / debt_amount;
                
                // Check if below liquidation threshold
                current_ratio < liquidation_ratio
            }
            _ => false, // Can't liquidate without price data
        }
    }
}
```

## 🔧 Advanced Usage

### Working with Decimals

Reflector prices use a specific decimal precision. Always check the `decimals()` function:

```rust
pub fn get_actual_price(
    env: Env,
    oracle_address: Address,
    asset: Symbol,
) -> f64 {
    let oracle = ReflectorOracleClient::new(&env, &oracle_address);
    
    // Get decimals and price
    let decimals = oracle.decimals();
    let (raw_price, _) = oracle.lastprice(&asset).unwrap();
    
    // Convert to actual price
    let divisor = 10_i128.pow(decimals as u32);
    (raw_price as f64) / (divisor as f64)
}
```

### Time-Weighted Average Price (TWAP)

```rust
pub fn calculate_twap(
    env: Env,
    oracle_address: Address,
    asset: Symbol,
    period_minutes: u32,
) -> Option<i128> {
    let oracle = ReflectorOracleClient::new(&env, &oracle_address);
    
    // Get historical prices (last N records)
    let records = period_minutes / 5; // 5-minute intervals
    let prices = oracle.prices(&asset, records);
    
    if prices.is_empty() {
        return None;
    }
    
    // Calculate weighted average
    let mut total_weighted_price = 0i128;
    let mut total_time = 0u64;
    
    for i in 1..prices.len() {
        let (price, timestamp) = prices.get(i).unwrap();
        let (_, prev_timestamp) = prices.get(i - 1).unwrap();
        
        let time_diff = timestamp - prev_timestamp;
        total_weighted_price += price * (time_diff as i128);
        total_time += time_diff;
    }
    
    if total_time > 0 {
        Some(total_weighted_price / (total_time as i128))
    } else {
        None
    }
}
```

## 📊 Testing Your Contract

### Unit Tests

```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_price_reader() {
        let env = Env::default();
        let contract_id = env.register(PriceReader, ());
        let client = PriceReaderClient::new(&env, &contract_id);
        
        // Mock oracle address (in real tests, deploy a mock oracle)
        let oracle_address = Address::generate(&env);
        let asset = symbol_short!("XLM");
        
        // Test price reading
        let price = client.get_price(&oracle_address, &asset);
        
        // Add your assertions here
        assert!(price.is_some());
    }
}
```

### Integration Tests

```bash
# Build your contract
soroban contract build

# Deploy to testnet
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/hello_reflector.wasm \
    --source alice \
    --network testnet

# Test with real Reflector oracle
soroban contract invoke \
    --id YOUR_CONTRACT_ID \
    --source alice \
    --network testnet \
    -- get_price \
    --oracle_address REFLECTOR_ORACLE_ADDRESS \
    --asset XLM
```

## 🎯 Best Practices

### 1. Always Check Price Freshness

```rust
fn is_price_fresh(timestamp: u64, max_age_seconds: u64) -> bool {
    let current_time = env.ledger().timestamp();
    current_time - timestamp <= max_age_seconds
}
```

### 2. Handle Missing Data Gracefully

```rust
pub fn get_price_with_fallback(
    env: Env,
    primary_oracle: Address,
    fallback_oracle: Address,
    asset: Symbol,
) -> Option<i128> {
    // Try primary oracle first
    if let Some(price) = get_price_from_oracle(&env, &primary_oracle, &asset) {
        return Some(price);
    }
    
    // Fallback to secondary oracle
    get_price_from_oracle(&env, &fallback_oracle, &asset)
}
```

### 3. Implement Circuit Breakers

```rust
pub fn safe_price_update(
    env: Env,
    oracle_address: Address,
    asset: Symbol,
    last_known_price: i128,
    max_deviation_percent: i128,
) -> Result<i128, &'static str> {
    let oracle = ReflectorOracleClient::new(&env, &oracle_address);
    
    if let Some((new_price, _)) = oracle.lastprice(&asset) {
        // Check for extreme price movements
        let deviation = ((new_price - last_known_price).abs() * 100) / last_known_price;
        
        if deviation > max_deviation_percent {
            Err("Price deviation too large")
        } else {
            Ok(new_price)
        }
    } else {
        Err("No price data available")
    }
}
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

### Debugging Tips

```rust
// Add logging for debugging
pub fn debug_price_info(env: Env, oracle_address: Address, asset: Symbol) {
    let oracle = ReflectorOracleClient::new(&env, &oracle_address);
    
    if let Some((price, timestamp)) = oracle.lastprice(&asset) {
        env.logs().add("Price found", &price);
        env.logs().add("Timestamp", &timestamp);
        env.logs().add("Current time", &env.ledger().timestamp());
    } else {
        env.logs().add("No price data", &asset);
    }
}
```

## 📚 Resources

### Official Documentation
- [Reflector Network](https://reflector.network/)
- [Soroban Documentation](https://soroban.stellar.org/)
- [SEP-40 Standard](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0040.md)

### Oracle Addresses

#### Testnet
```
XLM/USD Oracle: CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
BTC/USD Oracle: CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
ETH/USD Oracle: CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

#### Mainnet
```
XLM/USD Oracle: CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
BTC/USD Oracle: CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
ETH/USD Oracle: CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

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

---

*This tutorial is maintained by the community. For updates and contributions, visit our [GitHub repository](https://github.com/your-repo/reflector-tutorial).*