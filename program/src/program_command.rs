
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Command {
    Deposit,
    Withdraw,
    /// Create order
    CreateOrder,
    /// Cancel order
    CancelOrder,
    /// update order from oracle
    UpdateOrder,
    /// unlock traders coins and send them
    Unlock,
    /// set new owner
    SetOwner,
}