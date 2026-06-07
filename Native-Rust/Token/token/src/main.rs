use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    program_pack::Pack,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_sdk::system_instruction::create_account;
use spl_token::{id as token_program_id, instruction::initialize_mint, state::Mint};

#[tokio::main] 
async fn main() -> Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("http://localhost:8899"),
        CommitmentConfig::confirmed(),
    );
    let fee_payer = Keypair::new();
    let mint = Keypair::new();

    let airdrop_signature = client
        .request_airdrop(&fee_payer.pubkey(), 5_000_000_000) // req for 5 sol 
        .await?;

    client.confirm_transaction_with_commitment(&airdrop_signature, CommitmentConfig::confirmed()).await?;
    println!("Airdrop Confirmed.");

    let mint_rent = client
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
        .await?; 
    
    let latest_blockhash = client.get_latest_blockhash().await?;

    let transaction = Transaction::new_signed_with_payer(
        &[
            create_account(
                &fee_payer.pubkey(),
                &mint.pubkey(),
                mint_rent,
                Mint::LEN as u64,
                &token_program_id(),
            ),
            initialize_mint(
                &token_program_id(),
                &mint.pubkey(),
                &fee_payer.pubkey(),
                Some(&fee_payer.pubkey()),
                9
            )?,
        ],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &mint],
        latest_blockhash,
    );

    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;
    let mint_account = client.get_account(&mint.pubkey()).await?;
    let mint_data = Mint::unpack(&mint_account.data)?;

    println!("Mint address : {} ", mint.pubkey());
    println!("mint account : {:?} ", mint_data);
    println!("transaction_signature : {} ", transaction_signature);

    Ok(())
}