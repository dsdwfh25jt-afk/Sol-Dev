// functionalities in wallet 

// add acc , create acc
// send , recieve tokens 
use std::io::{self, Write};
use std::fs;
use std::path::PathBuf;
use std::path::Path;
use solana_sdk::signer::{keypair::{Keypair, keypair_from_seed}, Signer };
use anyhow::Result;
use bip39::{Language, Mnemonic};

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    // creating new keypair / account 
    let keypair = Keypair::new();
    println!("Public Key: {}", keypair.pubkey());
    println!("Secret Key: {:?}", keypair.to_bytes());
    save_key_to_project();
    import_from_seed(taking_seed_input());
    Ok(())
}


// creating a new acc and saving to the wallet.json file 
fn save_key_to_project() {
    let keypair = Keypair::new();
    println!("Public Key: {}", keypair.pubkey());
    println!("Secret Key: {:?}", keypair.to_bytes());


    // 1. Define the file name in the current project directory
    let file_path = Path::new("wallet_key.json");

    // 2. Define your key data 
    let secreat_key = keypair.to_bytes(); 

    // 3. Write the data to the file
    match fs::write(&file_path, secreat_key) {
        Ok(_) => println!("Successfully created and saved key to {:?}", file_path),
        Err(e) => println!("Failed to save the key: {}", e),
    }

}

//  importing wallet from mnenommics 
    fn import_from_seed(input_seed : String) {
    let phrase = input_seed;
    let mnemonic = Mnemonic::parse_in(Language::English, phrase).expect("Invalid mnemonic phrase entered!");

    let seed = mnemonic.to_seed("");
    let keypair = keypair_from_seed(seed.as_ref()).unwrap();

    println!("recovered address {:?}", keypair.pubkey());
    }

// taking seed input to recover wallet and returning the seed to the seed importing function 
fn taking_seed_input() -> String{
    print!("Please enter your 12 or 24-word mnemonic phrase: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let phrase = input.trim();

    println!("You entered: {}", phrase); 
    
    phrase.to_string()
}