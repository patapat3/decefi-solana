use crate::error::DecefiError;
use bytemuck::{bytes_of, cast};
#[cfg(test)]
use serde::{Deserialize, Serialize};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use arrayref::{array_ref, array_refs};
use std::num::NonZeroU64;

#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use proptest_derive::Arbitrary;

pub mod dcfi_token {
    use solana_sdk::declare_id;
    declare_id!("BQcdHdAQW1hczDbBi9hiegXAR7A98Q9jx3X3iBBBDiq4");
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary, Serialize, Deserialize))]
pub struct Deposit {
    #[cfg_attr(
    test,
    proptest(strategy = "(1u64..=std::u64::MAX).prop_map(|x| NonZeroU64::new(x).unwrap())")
    )]
    pub amount: NonZeroU64,
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary, Serialize, Deserialize))]
pub struct Withdraw {
    #[cfg_attr(
    test,
    proptest(strategy = "(1u64..=std::u64::MAX).prop_map(|x| NonZeroU64::new(x).unwrap())")
    )]
    pub amount: NonZeroU64,
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary, Serialize, Deserialize))]
pub struct NewOrderInstruction {
    pub hash: [u8; 256],
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary, Serialize, Deserialize))]
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
pub struct CancelOrderInstruction {
    pub order_id: u128,
    pub owner: [u64; 4], // Unused
    pub owner_slot: u8,
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary, Serialize, Deserialize))]
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
pub enum DecefiInstruction {
    Deposit(NonZeroU64),
    Withdraw(NonZeroU64),
    NewOrder(NewOrderInstruction),
    CancelOrder(CancelOrderInstruction),
    // CancelOrderByClientId(u64),
}

impl DecefiInstruction {
    #[cfg(test)]
    #[inline]
    pub fn serde_pack(&self) -> Vec<u8> {
        bincode::serialize(&(0u8, self)).unwrap()
    }

    pub fn pack(&self) -> Vec<u8> {
        bincode::serialize(&(0u8, self)).unwrap()
    }

    pub fn unpack(versioned_bytes: &[u8]) -> Option<Self> {
        if versioned_bytes.len() < 0 || versioned_bytes.len() > 512 { // #TODO: min/max?
            return None;
        }
        let (&[version], &discrim, data) = array_refs![versioned_bytes, 1, 4; ..;];
        if version != 0 {
            return None;
        }
        let discrim = u32::from_le_bytes(discrim);
        Some(match (discrim, data.len()) {
            (0, 8) => {
                let data_array = array_ref![data, 0, 8]; // u64 only
                let amount = NonZeroU64::new(u64::from_le_bytes(*data_array));
                amount
            },
            (1, 8) => {
                let data_array = array_ref![data, 0, 8]; // u64 only
                let amount = NonZeroU64::new(u64::from_le_bytes(*data_array));
                amount
            },
            (2, 256) => DecefiInstruction::NewOrder({
                let data_arr = array_ref![data, 0, 256]; // u8x256 order hash
                NewOrderInstruction { *hash }
            }),
            (3, 25) => DecefiInstruction::CancelOrder({
                let data_array = array_ref![data, 0, 25]; // order_hash, owner, owner_slot
                let fields = array_refs![data_array, 16, 8, 1];
                let order_id = u128::from_le_bytes(*fields.0);
                let owner = u64::from_le_bytes(*fields.1);
                let owner_slot = u8::from_le_bytes(*fields.2);
                CancelOrderInstruction {
                    order_id,
                    owner,
                    owner_slot,
                }
            }),
            _ => return None,
        })
    }

    #[cfg(test)]
    #[inline]
    pub fn unpack_serde(data: &[u8]) -> Result<Self, ()> {
        match data.split_first() {
            None => Err(()),
            Some((&0u8, rest)) => bincode::deserialize(rest).map_err(|_| ()),
            Some((_, _rest)) => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO add tests
}

#[cfg(feature = "fuzz")]
mod fuzzing {
    // TODO fix fuzz
}
