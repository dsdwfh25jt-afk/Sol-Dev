use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id:&Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult{
    msg!("Hello, World!");
    Ok(())
}

#[cfg(test)]
mod test {
    use litesvm::LiteSVM;
    use solana_sdk::{
        instruction::Instruction,
        message::Message,
        signature::{Keypair , Signer},
        transaction::Transaction,
    };
    #[test]
    fn test_hello_world() {
        let mut svm = LiteSVM::new();

        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();

        svm.add_program_from_file(program_id,"target/deploy/Hello_World.so").unwrap();

        let instruction = Instruction {
            program_id,
            accounts: vec![],
            data : vec![],
        };

        let message = Message::new(&[instruction], Some(&payer.pubkey()));
        let transaction = Transaction::new(&[&payer], message , svm.latest_blockhash());

        let result = svm.send_transaction(transaction);
        assert!(result.is_ok(), "Transaction Should succeed");
        let logs = result.unwrap().logs;
        println!("Logs : {logs:#?}");
    }
}
