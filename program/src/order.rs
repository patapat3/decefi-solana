
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum OrderState {
    Waiting,
    Processing,
    Finished,
    Dispute
}

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Order {
    pub state: OrderState,
    pub order_hash: [u8; 32],
    pub paid_back: u64,
    pub dcfi_reserved: u64
}