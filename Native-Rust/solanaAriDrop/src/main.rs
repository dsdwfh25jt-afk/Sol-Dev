use std::io;
use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{native_token::LAMPORTS_PER_SOL,signature::Signer, signer::keypair::Keypair};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899".to_string(),
        CommitmentConfig::confirmed(),
    );

    let wallet = Keypair::new();
    println!("Enter the amount of SOL tokens : ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;
     
    let sol : f64 = input.trim().parse().expect("Invalid number ");
    let lamps = (sol * LAMPORTS_PER_SOL as f64) as u64;

    let airdrop_signature = connection
        .request_airdrop(&wallet.pubkey(),lamps)
        .await?;

    // loop{
    //     let confirmed = connection.confirm_transaction(&airdrop_signature).await?;
    //     if confirmed {
    //         break;
    //     }
    // }

    // This replaces entire 'loop' block
    connection.poll_for_signature_confirmation(&airdrop_signature, 1).await?;

    let balance = connection.get_balance(&wallet.pubkey()).await?;
    println!("Balance : {} SOL", balance as f64 / LAMPORTS_PER_SOL as f64);
    println!("Wallet : {}",wallet.pubkey());
    println!("Air Drop Signeture : {airdrop_signature:#?}");

    Ok(())

}