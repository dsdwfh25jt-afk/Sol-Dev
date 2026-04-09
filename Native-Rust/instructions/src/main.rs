use anyhow::Result;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, 
    signature::Signer, 
    signer::keypair::Keypair
};
use solana_system_interface::instruction::transfer;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use std::io;

#[tokio::main]
async fn main() -> Result<()> {
    
    let sender = Keypair::new();
    let reciever = Keypair::new();

    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    ); 

    // let wallet = Keypair::new();
    println!("Enter the amount of SOL tokens : ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;
     
    let sol : f64 = input.trim().parse().expect("Invalid number ");
    let lamps = (sol * LAMPORTS_PER_SOL as f64) as u64;

    let airdrop_signature = connection
        .request_airdrop(&sender.pubkey(),lamps)
        .await?;

    connection.poll_for_signature_confirmation(&airdrop_signature, 1).await?;


    let transfer_amount = LAMPORTS_PER_SOL / 100;

    let transfer_instuction = transfer(
        &sender.pubkey(),
        &reciever.pubkey(), 
        transfer_amount,
    );

    println!("{:#?}", transfer_instuction);

    Ok(())
}