use borsh::{BorshDeserialize, BorshSerialize};
use crate::errors::LedgerError;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum LedgerInstruction {
    InitQuota { epoch: u32, daily_cap: u64 },
    ConsumeQuota { amount: u64 },
    ReplenishQuota { amount: u64 },
    SettleLamports { lamports: u64 },
    RegisterOperator { callsign: String, tier: u8 },
}

impl LedgerInstruction {
    pub fn encode(&self) -> Result<Vec<u8>, LedgerError> {
        borsh::to_vec(self).map_err(|_| LedgerError::MalformedPayload)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, LedgerError> {
        Self::try_from_slice(bytes).map_err(|_| LedgerError::MalformedPayload)
    }
}
