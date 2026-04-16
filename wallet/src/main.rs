// functionalities in wallet 

// add acc , create acc
// send , recieve tokens 

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
    import_from_seed();
     Ok(())
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

//  importing wallet from mnenommics 
    fn import_from_seed() {
    let phrase = "pill tomorrow foster begin walnut borrow virtual kick shift mutual shoe scatter";
    let mnemonic = Mnemonic::parse_in(Language::English, phrase).expect("Invalid mnemonic phrase entered!");

    let seed = mnemonic.to_seed("");
    let keypair = keypair_from_seed(seed.as_ref()).unwrap();

    println!("recovered address {:?}", keypair.pubkey());
    }



// use std::io::{self, Write};
// // Assuming you have your import_wallet_from_mnemonic function from earlier

// fn main() {
//     // 1. Print the prompt. We use print! instead of println! so the input 
//     // cursor stays on the same line.
//     print!("Please enter your 12 or 24-word mnemonic phrase: ");
    
//     // 2. We must flush stdout to ensure the print! macro displays immediately
//     io::stdout().flush().expect("Failed to flush stdout");

//     // 3. Create a mutable string to hold the user's input
//     let mut input = String::new();

//     // 4. Read the line from the terminal
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");

//     // 5. Trim the invisible newline characters (\n or \r\n) from the end
//     let phrase = input.trim();

//     println!("You entered: {}", phrase); // Just for testing! Remove in production.
    
//     // 6. Pass the sanitized phrase to your existing function
//     // import_wallet_from_mnemonic(phrase);
// }