use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    program_pack::Pack,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use spl_token_2022_interface::{
    id as token_2022__program_id, instruction::initialize_mint,state::Mint,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RpcClient::new_with_commitment(
        String::from("https://api.devnet.solana.com"),
        CommitmentConfig::confirmed(),
    );

    let recent_blockhash = client.get_latest_blockhash().await?;

    let fee_payer = Keypair::new();

    let aridrop_signature = client 
        .request_airdrop(&fee_payer.pubkey(),100_000_000_000)
        .await?;

    loop {
        let confirmed = client.confirm_transaction(&aridrop_signature).await?;
        if confirmed {
            break;
        }
    }

    let mint = Keypair::new();
    let space = Mint::LEN;
    let rent = client.get_minimum_balance_for_rent_exemption(space).await?;

    let create_account_instuction = create_account(
        &fee_payer.pubkey(),
        &mint.pubkey(),
        rent,
        space as u64,
        &token_2022__program_id(),
    );

    let initialize_mint_instruction = initialize_mint(
        &token_2022__program_id(),
        &mint.pubkey(),
        &fee_payer.pubkey(),
        Some(&fee_payer.pubkey()),
        9,
    )?;

    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instuction, initialize_mint_instruction],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &mint],
        recent_blockhash,
    );

    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;

    println!(" Mint Address : {}",mint.pubkey());
    println!("Transaction Signature : {}", transaction_signature);

    let account_info = client.get_account(&mint.pubkey()).await?;
    println!("{:#?}",account_info);

    let mint_account = Mint::unpack(&account_info.data)?;
    println!("{:#?}", mint_account);

    Ok(())
}

// dont know why but getting the some errros while compileing and running 
// after adding all the crates 