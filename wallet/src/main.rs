// functionalities in wallet 

// add acc , create acc
// send , recieve tokens 

use std::fs;
use std::path::PathBuf;
use std::path::Path;
use solana_sdk::signer::{keypair::Keypair, Signer};
// use anyhow::Result;
// use bip39::{Language, Mnemonic, Seed};

#[tokio::main]
async fn main() {

    // creating new keypair / account 
    let keypair = Keypair::new();
    println!("Public Key: {}", keypair.pubkey());
    println!("Secret Key: {:?}", keypair.to_bytes());
    save_key_to_project();
}


// creating a new acc
fn save_key_to_project() {
    let keypair = Keypair::new();
    println!("Public Key: {}", keypair.pubkey());
    println!("Secret Key: {:?}", keypair.to_bytes());


    // 1. Define the file name in the current project directory
    let file_path = Path::new("wallet_key.json");

    // 2. Define your dummy key data (usually formatted as a JSON array of bytes)
    let secreat_key = keypair.to_bytes(); 

    // 3. Write the data to the file
    match fs::write(&file_path, secreat_key) {
        Ok(_) => println!("Successfully created and saved key to {:?}", file_path),
        Err(e) => println!("Failed to save the key: {}", e),
    }



}
