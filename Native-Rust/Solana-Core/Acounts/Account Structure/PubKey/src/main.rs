use solana_sdk::signer::{keypair::Keypair, Signer};

#[tokio::main] 
async fn main() {
    let keypair = Keypair::new();
    println!("Public Key : {}",keypair.pubkey());
    println!("Secret Key : {:?}",keypair.to_bytes());
}

//  A  keypar has 
// Public key : used as address of account 
// private key : used to sign transaction  