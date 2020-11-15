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
    Deposit,
    Withdraw,
    NewOrder(NewOrderInstruction),
    CancelOrder(CancelOrderInstruction),
    CancelOrderByClientId(u64),
}

impl DecefiInstruction {
    #[cfg(test)]
    #[inline]
    pub fn serde_pack(&self) -> Vec<u8> {
        bincode::serialize(&(0u8, self)).unwrap()
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(43); // TODO fix packing/unpacking
        buf
    }

    pub fn unpack(versioned_bytes: &[u8]) -> Option<Self> {
        return None;
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
