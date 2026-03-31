use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

#[tokio::main]
async fn main() {
    let program_address = pubkey!("11111111111111111111111111111111");
    let seeds = [b"Hello World".as_ref()];
    let (pda,bump) = Pubkey::find_program_address(&seeds, &program_address); 

    println!("PDA : {}",pda);
    println!("Bump : {}",bump);

}

// PDA is derived from program ID and 1 or more optional seed 
// PDA dont have private key 

    