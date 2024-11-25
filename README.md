# Solana Jupiter Swap Integration

## Overview
A Rust implementation for performing token swaps on Solana using Jupiter's V6 API. This integration focuses on **SOL to SPL token swaps** (USDC by default, swapping the USDC mint address with the desired token address for other tokens) with optimized transaction settings and dynamic slippage protection.

## Features
- **SOL to token swaps** via Jupiter aggregator
- **Dynamic priority fees** with auto-multiplier
- **Optimized DEX routing** through:
  - Whirlpool
  - Meteora DLMM
  - Raydium CLMM
- **Dynamic slippage protection** (0.5% - 5%)
- **Dynamic compute unit limit estimation**
- **Environment variable configuration**
- **Transaction status monitoring** with Solana explorer integration

## Prerequisites
- Rust (latest stable version)
- Solana CLI tools
- Funded Solana wallet (keypair at `~/.config/solana/id.json`)

## Installation

1. **Clone the repository:**
   ```
   git clone https://github.com/Kyan148369/NFTSwap-.git
   cd solana-jupiter-swap
  

2.Install dependencies:
    ```
    Cargo build
    ```
3. Run the swap program

    ``` 
    Cargo run


Implementation Details
The program performs a SOL to USDC swap with the following specifications:

    ```
        let quote_request = QuoteRequest {
        amount: 1_000_000, // 0.001 SOL
        input_mint: NATIVE_MINT, // SOL
        output_mint: USDC_MINT, // USDC
        dexes: Some("Whirlpool,Meteora DLMM,Raydium CLMM".into()),
        slippage_bps: 50, // 0.5% base slippage
        ..QuoteRequest::default()
};

```

Transaction Configuration
The implementation uses optimized transaction settings:

```
    TransactionConfig {
        wrap_and_unwrap_sol: true,
        prioritization_fee_lamports: Some(PrioritizationFeeLamports::AutoMultiplier(2)),
        dynamic_compute_unit_limit: true,
        dynamic_slippage: Some(DynamicSlippageSettings {
            min_bps: Some(50), // 0.5% minimum slippage
            max_bps: Some(500) // 5% maximum slippage
        }),
        // ... other optimized defaults 
    }
Key Components
*Priority Fee System
    *Uses Auto-multiplier (2x) for dynamic fee calculation.
    *Automatically adjusts based on network conditions.
*Slippage Protection
    *Dynamic range: 0.5% - 5%.
    *Adjusts based on market conditions.
Transaction Monitoring
    *Success/failure status tracking.
    *Explorer integration.

