Solana Jupiter Swap Integration
A Rust implementation for performing token swaps on Solana using Jupiter's V6 API. This integration focuses on SOL to SPL token swaps(USDC by default swap USDC mint adress with the desired token address) with optimized transaction settings and dynamic slippage protection.
🚀 Features

SOL to token swaps via Jupiter aggregator
Dynamic priority fees with auto-multiplier
Optimized DEX routing through:

Whirlpool
Meteora DLMM
Raydium CLMM


Dynamic slippage protection (0.5% - 5%)
Dynamic compute unit limit estimation
Environment variable configuration
Transaction status monitoring with Solana explorer integration

📋 Prerequisites

Rust (latest stable version)
Solana CLI tools
Funded Solana wallet (keypair at ~/.config/solana/id.json)

🛠️ Installation

Clone the repository:

bashCopygit clone https://github.com/yourusername/solana-jupiter-swap
cd solana-jupiter-swap

Create a .env file:

envCopyAPI_BASE_URL=https://quote-api.jup.ag/v6

Install dependencies:

bashCopycargo build
💻 Usage
Run the swap program:
bashCopycargo run
Implementation Details
The program performs a SOL to USDC swap with the following specifications:
rustCopylet quote_request = QuoteRequest {
    amount: 1_000_000,  // 0.001 SOL
    input_mint: NATIVE_MINT,  // SOL
    output_mint: USDC_MINT,   // USDC
    dexes: Some("Whirlpool,Meteora DLMM,Raydium CLMM".into()),
    slippage_bps: 50,         // 0.5% base slippage
    ..QuoteRequest::default()
};
Transaction Configuration
The implementation uses optimized transaction settings:
rustCopyTransactionConfig {
    wrap_and_unwrap_sol: true,
    prioritization_fee_lamports: Some(PrioritizationFeeLamports::AutoMultiplier(2)),
    dynamic_compute_unit_limit: true,
    dynamic_slippage: Some(DynamicSlippageSettings {
        min_bps: Some(50),   // 0.5% minimum slippage
        max_bps: Some(500)   // 5% maximum slippage
    }),
    // ... other optimized defaults
}
⚙️ Key Components

Priority Fee System

Uses Auto-multiplier (2x) for dynamic fee calculation
Automatically adjusts based on network conditions


Slippage Protection

Dynamic range: 0.5% - 5%
Adjusts based on market conditions


DEX Integration

Whirlpool
Meteora DLMM
Raydium CLMM


Transaction Monitoring

Success/failure status tracking
Solana Explorer integration




📁 Project Structure
Copy.
├── src/
│   └── main.rs           # Main implementation file
├── Cargo.toml            # Dependencies and project config
├── .env                  # Environment variables
└── README.md            # Documentation


🤝 Contributing

Fork the repository
Create your feature branch
Commit your changes
Push to the branch
Open a Pull Request

⚠️ To Keep in Mind

Currently configured for SOL to USDC by default
Requires local keypair file for signature 
Uses public RPC endpoint you can swap out your api keys 