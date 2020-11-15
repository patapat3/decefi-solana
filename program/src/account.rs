#![no_std]
#![cfg(feature = "program")]

use crate::order::Order;
use std::collections::LinkedList;


impl Default for AccountOrders {
    fn default() -> Self {
        let mut list: LinkedList<Order> = LinkedList::new();
        AccountOrders{ orders: list }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountOrders {
    orders: LinkedList<Order>,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    orders: AccountOrders,
    dcfi_reserved: u64
}

impl Account {
    pub fn create_order(order: Order) -> bool {
        return true
    }
}