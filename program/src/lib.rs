#![allow(clippy::try_err)]

#[macro_use]
pub mod error;

#[macro_use]
extern crate serde_derive;
extern crate solana_sdk;

mod account;
mod order;
mod program_command;
mod instructions;
pub mod decefi;

use order::Order;
use order::OrderState;
use decefi::Decefi;

use byteorder::{ByteOrder, LittleEndian};
#[cfg(feature = "program")]
use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint_deprecated,
    entrypoint_deprecated::ProgramResult,
    info,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_sdk::instruction::Instruction;


// Declare and export the program's entrypoint
entrypoint_deprecated!(process_instruction);

// Program entrypoint's implementation
fn process_instruction<'a>(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &'a [AccountInfo<'a>], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    Ok(decefi::Decefi::process(
        program_id,
        accounts,
        _instruction_data,
    )?)
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_sdk::clock::Epoch;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u64>()];
        LittleEndian::write_u64(&mut data, 0);
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(LittleEndian::read_u64(&accounts[0].data.borrow()), 0);
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(LittleEndian::read_u64(&accounts[0].data.borrow()), 1);
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(LittleEndian::read_u64(&accounts[0].data.borrow()), 2);
    }
}

// Required to support info! in tests
#[cfg(not(target_arch = "bpf"))]
solana_sdk::program_stubs!();
