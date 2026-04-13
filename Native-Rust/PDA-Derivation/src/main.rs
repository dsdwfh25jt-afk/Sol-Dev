use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // owner of pda = program address 
    let program_address = Pubkey::from_str("11111111111111111111111111111111")?;

    // address seed uses for differentiate mutiple account's  pda 
    let optional_seed_add = Pubkey::from_str("B9Lf9z5BfNPT4d5KMeaBFx8x1G4CULZYR1jA2kmxRDkA")?;

    // another seed (string must in byte form)
    let byte_seeds = b"helloWorld";
    // let seed : &[&[u8]] = &[b"helloWorld"];  for only just single string seed 

    // generates seed from mutiple seed , actually its just concatination (byte_seed + optional_seed_add)
    let seeds : &[&[u8]] = &[byte_seeds, optional_seed_add.as_ref()];

    let (pda, bump) = Pubkey::find_program_address(seeds, &program_address);

    println!("PDA : {}",pda);
    println!("SEEDS : {}",bump);
    Ok(())
}