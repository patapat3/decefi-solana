use byteorder::{ByteOrder, LittleEndian};
use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint_deprecated,
    entrypoint_deprecated::ProgramResult,
    info,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_sdk::instruction::Instruction;

use crate::{
    error::{DecefiResult},
    instructions::{
        NewOrderInstruction, DecefiInstruction
    },
    program_command::Command,
    account::Account,
    order::{
        Order, OrderState
    }
};

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Decefi {
}

#[cfg_attr(not(feature = "program"), allow(unused))]
impl Decefi {
    #[cfg(feature = "program")]
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> DecefiResult {
        info!("decefi Rust program entrypoint");

        let command = DecefiInstruction::unpack(input)?;
        let account_info_iter = &mut accounts.iter();
        match command {
            DecefiInstruction::NewOrder(ref inner) => {
                let trader_account = next_account_info(account_info_iter)?;
                let mut account = Account::deserialize(&trader_account.data.borrow())?;
                account.create_order(Order {
                    state: OrderState::Waiting,
                    order_hash: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
                    paid_back: 0,
                    dcfi_reserved: 0
                })
            },
            DecefiInstruction::CancelOrder(ref inner)=>{
                return Ok(());
            }
            _ => {}
        }

        info!("Transaction processed.");

        Ok(())
    }
}