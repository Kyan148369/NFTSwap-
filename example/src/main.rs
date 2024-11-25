use std::env;
use dotenv::dotenv;
use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::{TransactionConfig,PrioritizationFeeLamports,DynamicSlippageSettings},
    JupiterSwapApiClient,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey, transaction::VersionedTransaction};
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use solana_sdk::signer::Signer;
use tokio;

const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

pub const TEST_WALLET: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm"); // Coinbase 2 wallet

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load the .env file
    let api_base_url = env::var("API_BASE_URL").unwrap_or("https://quote-api.jup.ag/v6".into());
    println!("Using base url: {}", api_base_url);

    let jupiter_swap_api_client = JupiterSwapApiClient::new(api_base_url);

    let quote_request = QuoteRequest {
        amount: 1_000_000,
        input_mint: NATIVE_MINT,
        output_mint: USDC_MINT,
        dexes: Some("Whirlpool,Meteora DLMM,Raydium CLMM".into()),
        slippage_bps: 50,
        ..QuoteRequest::default()
    };

    // GET /quote
    let quote_response = jupiter_swap_api_client.quote(&quote_request).await.unwrap();
    println!("{quote_response:#?}");

    // Load keypair from default Solana config location
    let keypair = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Failed to read keypair file");
    let wallet_pubkey = keypair.pubkey();

    // POST /swap
    let swap_response = jupiter_swap_api_client
        .swap(&SwapRequest {
            user_public_key: wallet_pubkey,
            quote_response: quote_response.clone(),
            config: TransactionConfig {
                // Keep default settings for most fields
            wrap_and_unwrap_sol: true,
            fee_account: None,
            destination_token_account: None,
            // Add prioritization fee (in lamports)
            prioritization_fee_lamports: Some(PrioritizationFeeLamports::AutoMultiplier(2)), // 0.001 SOL = 1,000,000 lamports
            // Enable dynamic compute unit limit for better estimation
            dynamic_compute_unit_limit: true,
            // Optionally, set compute unit price (in micro-lamports)
            compute_unit_price_micro_lamports: None,
            as_legacy_transaction: false,
            use_shared_accounts: true,
            use_token_ledger: false,
            allow_optimized_wrapped_sol_token_account: false,
            tracking_account: None,
            skip_user_accounts_rpc_calls: false,
            keyed_ui_accounts: None,
            program_authority_id: None,
            dynamic_slippage: Some(DynamicSlippageSettings {
                min_bps: Some(50),
                max_bps: Some(500)
            }),
        },

            
        })
        .await
        .unwrap();

    println!("Raw tx len: {}", swap_response.swap_transaction.len());

    let versioned_transaction: VersionedTransaction =
        bincode::deserialize(&swap_response.swap_transaction).unwrap();

    // Replace null_signer section with real signing
    let signed_versioned_transaction =
        VersionedTransaction::try_new(versioned_transaction.message, &[&keypair]).unwrap();

    // send with rpc client...
    
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".into());


    // This will fail with "Transaction signature verification failure" as we did not really sign
    // Send and confirm transaction, handling both success and error cases
    match rpc_client
        .send_and_confirm_transaction(&signed_versioned_transaction)
        .await
    {
        Ok(signature) => {
            println!("Transaction successful!");
            println!("Signature: {}", signature);
            println!("View transaction: https://solscan.io/tx/{}", signature);
        },
        Err(error) => {
            println!("Transaction failed: {}", error);
        }
    }
}



