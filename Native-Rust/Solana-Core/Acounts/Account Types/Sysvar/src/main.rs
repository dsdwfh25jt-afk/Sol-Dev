use anyhow::Result;
use bincode::deserialize; // this is abandoned project is no longer maintained use wincode instead or postcard or rkyv
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::sysvar::{self, clock::Clock};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = RpcClient::new_with_commitment(
        "https://api.mainnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let account = connection.get_account(&sysvar::clock::ID).await?;

    let clock : Clock = deserialize(&account.data)?;

    println!("{:#?}",account);
    println!("{:#?}",clock);

    Ok(())
}


// output : 
// Account {
//     lamports: 1169280,
//     data.len: 40,
//     owner: Sysvar1111111111111111111111111111111111111,
//     executable: false,
//     rent_epoch: 18446744073709551615,
//     data: c3367218000000004403cb6900000000b503000000000000b603000000000000d807cc6900000000, 

                // this data is address of data account which is storing data of clock 



// }
// Clock {
//     slot: 410138307,
//     epoch_start_timestamp: 1774912324,
//     epoch: 949,
//     leader_schedule_epoch: 950,
//     unix_timestamp: 1774979032,
// }