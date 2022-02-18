pub mod error;
pub mod processor;

use {
    std::convert::TryInto,
    crate::{error::CustomError},
    solana_program::{
        account_info::AccountInfo,
        entrypoint,
        entrypoint::ProgramResult,
        program_error::PrintProgramError,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: u8 = instruction_data[0];

    if let Err(error) = match instruction {
        1  => processor::castVote(program_id, accounts, u64::from_le_bytes(instruction_data[1..9].try_into().unwrap()), u64::from_le_bytes(instruction_data[9..17].try_into().unwrap())),
        2  => processor::createVote(program_id, accounts, u64::from_le_bytes(instruction_data[1..9].try_into().unwrap()), &instruction_data[9..]),
        _ => Err(ProgramError::InvalidInstructionData)
    } {
        error.print::<CustomError>();
        return Err(error);
    }

    Ok(())
}
