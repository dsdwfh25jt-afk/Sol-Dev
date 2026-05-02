use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

entrypoint!(process_instruction);

#[derive(BorshDeserialize)]
enum ProgramInstruction{
    SolTransfer { amount : u64 }, 

}

impl ProgramInstruction {
    fn unpack(inout: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}

pub fn process_instruction(
    program_id : &pubkey,
    accounts : &[AccountInfo],
    instruction_data : &[u8],
) -> ProgramResult {
    let instruction = ProgramInstruction::unpack(instruction_data)?;

    match instruction {
        ProgramInstruction::SolTransfer { amount } => {
            let [pda_account_info, recipient_info, system_program_info] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };

            let recipient_pubkey = recipient_info.key;
            let seeds = &[b"pda", recipient_pubkey.as_ref()];
            let (expected_pda , bump_seed) = Pubkey::find_program_address(seeds, program_id);

            if expected_pda != *pda_account_info.key {
                return Err(ProgramError::InvalidArgument);
            }

            let transfer_ix = system_instruction::transafer(
                pda-account_info.key,
                recipient_info.key,
                amount, 
            );

            let singer_seeds : &[&[&[u8]]] = &[&[b"pda", recipient_pubkey.as_ref(), &[bump_seed]]];

            invoke_signed(
                &transfer_ix,
                &[
                    pda_account_info.clone(),
                    recipient_info.clone(),
                    system_program_info.clone(),
                ],
                singer_seeds,
            )?;

            Ok(())
        }
    }
}
