use borsh::BorshDeserialize;
use solana_program:: {
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

entrypoint!(process_instruction);

#[derive(BorshDeserialize)]
enum ProgramInstruction {
    SolTransfer{amount: u64},
}

impl ProgramInstruction {
    fn unpack(input: &[u8]) -> Result<Self , ProgramError> {
        Self::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}

pub fn process_instruction (
    _program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data:&[u8],
) -> ProgramResult {
    
    let instruction = ProgramInstruction::unpack(instruction_data)?;


    match instruction {
        ProgramInstruction::SolTransfer{ amount } => {
            let [sender_info, recipient_info, system_program_info] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };

            if !sender_info.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            };

            let transfer_ix = system_instruction::transfer(
                sender_info.key,
                recipient_info.key,
                amount,
            );

            invoke(
                &transfer_ix,
                &[
                    sender_info.clone(),
                    recipient_info.clone(),
                    system_program_info.clone(),
                ],
            )?;
            Ok(())
        }
    }
}

fn main() {
}