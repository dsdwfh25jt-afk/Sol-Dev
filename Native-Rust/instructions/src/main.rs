use anyhow::Result;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, 
    signature::Signer, 
    signer::keypair::Keypair
};
use solana_system_interface::instruction::transfer;

#[tokio::main]
async fn main() -> Result<()> {
    
    let sender = Keypair::new();
    let reciever = Keypair::new();

    let transfer_amount = LAMPORTS_PER_SOL / 100;

    let transfer_instuction = transfer(
        &sender.pubkey(),
        &reciever.pubkey(), 
        transfer_amount,
    );

    println!("{:?}", transfer_instuction);

    Ok(())
}