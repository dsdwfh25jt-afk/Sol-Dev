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

// this entrypoint macro is used as main function which shows the 
// starting point 
entrypoint!(process_instruction);

// it converting the row data from vec[u8] format to data format 
// and enum contains the instrutions from which only one at a time can be 
// performed 
#[derive(BorshDeserialize)]
enum ProgramInstruction {
    SolTransfer{amount: u64},
}

// it is simply parsing the data it means it converting the unwated string data into 
// numbers and if they are letters then simmpy return invalid intruction data 
impl ProgramInstruction {
    fn unpack(input: &[u8]) -> Result<Self , ProgramError> {
        Self::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}

// this fucntion named process instructions getting the nessesary data as args
// to perform instructions 
// program id 
// accounts  ... to and from 
// intruction data ... what instruction and how much (if its transafer) 
pub fn process_instruction (
    _program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data:&[u8],
) -> ProgramResult {
    
    // parsing the data of intruction if its not number then return err 
    let instruction = ProgramInstruction::unpack(instruction_data)?;
    
    // validating the instruction data doing security checks on the data 
    match instruction {
        ProgramInstruction::SolTransfer{ amount } => {

            // cheching sender , reciever and system programs are accounts or not 
            // if not return error and fail the transaction 
            let [sender_info, recipient_info, system_program_info] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };
            // checking whether the sender is the signer or not 
            // if sender is not signer means someone else is trying to perform the instruction 
            // and if its true then return error and fail the transaction 
            if !sender_info.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            };

            // this is the transfer instructions data which will later provided to the system 
            // program and this has been validated erlier 
            let transfer_ix = system_instruction::transfer(
                sender_info.key,
                recipient_info.key,
                amount,
            );
            
            // this is system program invoke function which will do the transaction 
            // this take sender acc , reciever acc and system program acc and 
            // the system program will do transfer the sols 
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