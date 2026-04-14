use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    let program_add = Pubkey::from_str("11111111111111111111111111111111")?;
    let optional_seed_bytes = b"hi there i am sol dev";
    let optional_seed_add = Pubkey::from_str("B9Lf9z5BfNPT4d5KMeaBFx8x1G4CULZYR1jA2kmxRDka")?;
    // let seeds : &[u8] = optional_seed_bytes,optional_seed_add.as_ref();
    // let (pda, bump) = Pubkey::find_program_address(seeds , &program_address);?

    for bump in (0..=255).rev() {
        match Pubkey::create_program_address(&[optional_seed_add.as_ref(), optional_seed_bytes , &[bump]], &program_add) {
            Ok(pda) => println!("bump {}: {}",bump , pda),
            Err(err) => println!("bump {}: {}",bump ,err),
        }
    }

    Ok(())

}


// Always use the canonical bump when deriving PDAs. Using a non-canonical bump creates a second valid address for the same seeds, which can lead to vulnerabilities where an attacker substitutes a different account than expected.
// canonical means the first valid add from seed and program add and the bump from 255 to 0 