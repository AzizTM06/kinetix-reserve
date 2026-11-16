use borsh::{BorshDeserialize, BorshSerialize};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Address(pub [u8; 32]);

impl Address {
    pub fn from_seed(tag: u8) -> Self {
        let mut raw = [0u8; 32];
        raw[0] = tag;
        raw[31] = tag.wrapping_mul(17);
        Self(raw)
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}{:02x}..{:02x}{:02x}", self.0[0], self.0[1], self.0[30], self.0[31])
    }
}

#[derive(Debug, Clone)]
pub struct SimAccount {
    pub address: Address,
    pub owner: Address,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl SimAccount {
    pub fn new(address: Address, owner: Address, lamports: u64, space: usize, is_signer: bool, is_writable: bool) -> Self {
        Self {
            address,
            owner,
            lamports,
            data: vec![0u8; space],
            is_signer,
            is_writable,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct EpochQuotaState {
    pub initialized: bool,
    pub operator: Address,
    pub epoch: u32,
    pub daily_cap: u64,
    pub consumed: u64,
    pub settlements_count: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct OperatorRegistryEntry {
    pub initialized: bool,
    pub authority: Address,
    pub callsign: String,
    pub tier: u8,
}
