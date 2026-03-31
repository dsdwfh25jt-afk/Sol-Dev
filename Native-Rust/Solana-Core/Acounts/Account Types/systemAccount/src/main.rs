use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    signer::{keypair::Keypair, Signer},
};

#[tokio::main]
async fn main() -> Result<()> {
    let keypair = Keypair::new();
    println!("Public key : {}",keypair.pubkey());

    let connection = RpcClient::new_with_commitment(
        // created small mistake called https server instead of http for local validator 
        // and solved and learned something new from that silly mess 
        "http://localhost:8899".to_string(),
        CommitmentConfig::confirmed(),
    );

    let signature = connection 
        .request_airdrop(&keypair.pubkey(), LAMPORTS_PER_SOL)
        .await?;

    loop {
        let confirmed = connection.confirm_transaction(&signature).await?;
        if confirmed{
            break;
        }
    }

    // created a user wallet , airdroped some sol in it , and printed its info 
    // and learned about system owned accounts 
    let account_info = connection.get_account(&keypair.pubkey()).await?;
    println!("{:#?}",account_info);
    Ok(())
}