use soroban_sdk::{contracttype, Address};

/// Persistent storage keys
#[contracttype]
pub enum DataKey {
    /// The SEP-41 token contract this transfer contract operates on
    TokenContract,
    /// Admin address allowed to update the token contract
    Admin,
}

/// Event emitted after every successful transfer
#[contracttype]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    /// Optional opaque reference (e.g. chat message ID) stored as bytes
    pub memo: soroban_sdk::Bytes,
}
