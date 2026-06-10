use soroban_sdk::{contracttype, BytesN};

#[contracttype]
#[derive(Clone, Debug)]
pub struct EscrowRecord {
    pub invoice_id: BytesN<32>,
    pub amount: u128,
    pub locked_at: u64,
}

#[contracttype]
pub enum DataKey {
    Admin,
    PoolContract,
    InvoiceContract,
    UsdcAsset,
    Locked(BytesN<32>),
}
