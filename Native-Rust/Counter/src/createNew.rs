use solana_sdk::{signature::Keypair, signer::Signer};

pub fn newWallet()  {
    let keypair = Keypair::new();
    let address = keypair.pubkey();
    println!("Account Created SuccessFully .");
    println!("Address: {address}");
}