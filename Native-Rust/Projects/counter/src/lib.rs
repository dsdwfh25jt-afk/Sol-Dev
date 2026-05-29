use borsh::{BorshDeserialize, BorshSerialize}; // byte form to actual data and vice versa 
use solana_program::{
    account_info::{next_account_info, AccountInfo}, // contain acc metadata containing stucts 
    declare_id,  // define program_id 
    entrypoint::ProgramResult, // returns result of success or failed  
    msg, // this is macro to print stuff specifically for solana
    program_error::ProgramError, // this contains standard solana errors like insf args , insf funds and so on 
    pubkey::Pubkey, // a byte array that forms pubkey 
};

mod state;
pub use state::*; // adding state of acc 

declare_id!(""); // get program id from .so file which generated after program building 
// program building is also a deep task which goes throgh multiple stages : 
// 1) Rust user level : the code we are typing or generating here 
// 2) rustc compiler compiles the code 
// 3) MIR (mid-level instruction reprentation) check add the memory flows and lifetime and safty 
// 4) LLVM IR (Low Level Vertual Machine Instuction Representation) convert the rust instruction into 
// machine level language like assembly 
// 5) LLVM eBPF (Low Level Vertual Machine extended Barkley Packet Format) this low level now get into 
// barkley packet format which is used to one program on multiple varous machines 
// 6) sBPF (solana Barkley Packet Format) and at the end that all converted into solana specific 
// code format which we can .so file that is what stored on validator end for fast execution 

#[cfg(not(feature = "no-entrypoint"))] // yet to understand this 
use solana_program::entrypoint; // using entrypoint macro from solana_program crate 
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction); // this is the entry point of all rust solana programs 


// this is the main fucntion of the program which is passed into the entrypoint macro 
// and will be called by the client side 
pub fn process_instruction (
    _program_id: &Pubkey, // getting the program id from client side as args 
    accounts: &[AccountInfo], // gettings acc info from client side as args ( such as which acc trasffering whome are they writteable and so on)
    intruction_data: &[u8], // --//-- which opetation are needed to perform 

    // getting byte code (memory level) refference of all of these 
) -> ProgramResult {
    let ( instruction_descriminant, instruction_data_inner ) = instruction_data.split_at(1); // matching the instruction whith values 
    // need more deeper understading on instruction level stuff
    match instruction_descriminant[0] { // matching the first byte add if 0 means true and do this 
        0 => {
            msg!("Instruction: Increament"); // printing this msg 
            process_increament_counter( accounts , instruction_data_inner)?; // calling fn p-i-c and passing args 
            // accounts which got from client side and instruction_data_inner which got from instruction data after calling split_at(1) (yet to understand this fn) 
        }
        _ => { // anything except 0 return error in result 
            msg!("Error : unknown instruction")
        }
    }
    Ok(()) // if it comes till here which means work done successfully 
}

pub fn process_increament_counter( 
    accoutns: &[AccountInfo], // getting acc from process instruction fn 
    _instruction_data: &[u8], // getting instruction data from --//-- 
) -> Result<(), ProgramError> { // returning result type 
    let account_info_iter = &mut accounts.iter(); // iterating over accnts 

    let counter_account = next_acccount_info(account_info_iter)?; // yet to understand this fn
    assert!(
        counter_account.is_writable, // verifying is account writeable 
        "Counter account must be writable"
    );

    let mut counter = Counter::try_from_slice(&counter_account.try_borrow_mut_data()?)?; // deserializing data  from counter acc 
    counter.count += 1; // adding one to counter.count can be used .checked_add(1) for safety 
    counter.serialize(&mut *counter_account.data.borrow_mut())?; // now serializing the data 
    // and now the update has been successfully done 

    msg!("Counter state Increament to {:?}" , counter.count); // printing the latest value of count variable  
    Ok(()) // returning succssfull 
}