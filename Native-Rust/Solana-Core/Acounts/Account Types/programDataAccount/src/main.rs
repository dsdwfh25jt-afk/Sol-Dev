use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey;

#[tokio::main]
async fn main() -> Result<()> {
    // connected to the solana mainnet 
    let connection = RpcClient::new_with_commitment(
        "https://api.mainnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    // converted this 32byte program id into byte code 
    let program_id = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"); 

    // from mainnet fetched data account from byte coded program id 
    let account_info = connection.get_account(&program_id).await?; 

    // simply printed account info 
    println!("{:#?}",account_info);

    Ok(())
}

// out put 

// Account {
//     lamports: 5595651284,
//     data.len: 134080,
//     owner: BPFLoader2111111111111111111111111111111111,
//     executable: true,
//     rent_epoch: 18446744073709551615,
//     data: 7f454c460201010000000000000000000300f70001000000d8f90000000000004000000000000000800902000000000000000000400038000400400009000800,
// }