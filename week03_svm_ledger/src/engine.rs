use borsh::BorshDeserialize;
use crate::errors::LedgerError;
use crate::instructions::LedgerInstruction;
use crate::ledger::{Address, EpochQuotaState, OperatorRegistryEntry, SimAccount};

pub struct LedgerEngine;

impl LedgerEngine {
    pub fn execute(
        program_id: &Address,
        accounts: &mut [SimAccount],
        raw_instruction: &[u8],
    ) -> Result<(), LedgerError> {
        let ix = LedgerInstruction::decode(raw_instruction)?;
        match ix {
            LedgerInstruction::InitQuota { epoch, daily_cap } => {
                Self::init_quota(program_id, accounts, epoch, daily_cap)
            }
            LedgerInstruction::ConsumeQuota { amount } => {
                Self::consume_quota(program_id, accounts, amount)
            }
            LedgerInstruction::ReplenishQuota { amount } => {
                Self::replenish_quota(program_id, accounts, amount)
            }
            LedgerInstruction::SettleLamports { lamports } => {
                Self::settle_lamports(accounts, lamports)
            }
            LedgerInstruction::RegisterOperator { callsign, tier } => {
                Self::register_operator(program_id, accounts, callsign, tier)
            }
        }
    }

    fn init_quota(
        program_id: &Address,
        accounts: &mut [SimAccount],
        epoch: u32,
        daily_cap: u64,
    ) -> Result<(), LedgerError> {
        if accounts.len() < 2 {
            return Err(LedgerError::MissingAccounts);
        }
        let (quota_acc, rest) = accounts.split_at_mut(1);
        let quota_acc = &mut quota_acc[0];
        let operator_acc = &rest[0];

        if !operator_acc.is_signer {
            return Err(LedgerError::MissingSignature);
        }
        if !quota_acc.is_writable {
            return Err(LedgerError::ReadOnlyAccount);
        }
        if &quota_acc.owner != program_id {
            return Err(LedgerError::OwnerMismatch);
        }

        let state = EpochQuotaState {
            initialized: true,
            operator: operator_acc.address,
            epoch,
            daily_cap,
            consumed: 0,
            settlements_count: 0,
        };
        let encoded = borsh::to_vec(&state).map_err(|_| LedgerError::MalformedPayload)?;
        if encoded.len() > quota_acc.data.len() {
            quota_acc.data.resize(encoded.len(), 0);
        }
        quota_acc.data[..encoded.len()].copy_from_slice(&encoded);
        Ok(())
    }

    fn consume_quota(
        program_id: &Address,
        accounts: &mut [SimAccount],
        amount: u64,
    ) -> Result<(), LedgerError> {
        if accounts.len() < 2 {
            return Err(LedgerError::MissingAccounts);
        }
        let (quota_acc, rest) = accounts.split_at_mut(1);
        let quota_acc = &mut quota_acc[0];
        let operator_acc = &rest[0];

        if !operator_acc.is_signer {
            return Err(LedgerError::MissingSignature);
        }
        if !quota_acc.is_writable {
            return Err(LedgerError::ReadOnlyAccount);
        }
        if &quota_acc.owner != program_id {
            return Err(LedgerError::OwnerMismatch);
        }

        let mut cursor: &[u8] = &quota_acc.data;
        let mut state = EpochQuotaState::deserialize(&mut cursor)
            .map_err(|_| LedgerError::MalformedPayload)?;
        if state.operator != operator_acc.address {
            return Err(LedgerError::MissingSignature);
        }

        let next_consumed = state
            .consumed
            .checked_add(amount)
            .ok_or(LedgerError::ArithmeticBoundary)?;
        if next_consumed > state.daily_cap {
            return Err(LedgerError::QuotaCapExceeded);
        }

        state.consumed = next_consumed;
        state.settlements_count = state
            .settlements_count
            .checked_add(1)
            .ok_or(LedgerError::ArithmeticBoundary)?;

        let encoded = borsh::to_vec(&state).map_err(|_| LedgerError::MalformedPayload)?;
        quota_acc.data[..encoded.len()].copy_from_slice(&encoded);
        Ok(())
    }

    fn replenish_quota(
        program_id: &Address,
        accounts: &mut [SimAccount],
        amount: u64,
    ) -> Result<(), LedgerError> {
        if accounts.len() < 2 {
            return Err(LedgerError::MissingAccounts);
        }
        let (quota_acc, rest) = accounts.split_at_mut(1);
        let quota_acc = &mut quota_acc[0];
        let operator_acc = &rest[0];

        if !operator_acc.is_signer {
            return Err(LedgerError::MissingSignature);
        }
        if &quota_acc.owner != program_id {
            return Err(LedgerError::OwnerMismatch);
        }

        let mut cursor: &[u8] = &quota_acc.data;
        let mut state = EpochQuotaState::deserialize(&mut cursor)
            .map_err(|_| LedgerError::MalformedPayload)?;

        state.consumed = state
            .consumed
            .checked_sub(amount)
            .ok_or(LedgerError::ArithmeticBoundary)?;

        let encoded = borsh::to_vec(&state).map_err(|_| LedgerError::MalformedPayload)?;
        quota_acc.data[..encoded.len()].copy_from_slice(&encoded);
        Ok(())
    }

    fn settle_lamports(accounts: &mut [SimAccount], lamports: u64) -> Result<(), LedgerError> {
        if accounts.len() < 2 {
            return Err(LedgerError::MissingAccounts);
        }
        let (left, right) = accounts.split_at_mut(1);
        let source = &mut left[0];
        let destination = &mut right[0];

        if !source.is_signer {
            return Err(LedgerError::MissingSignature);
        }
        if !source.is_writable || !destination.is_writable {
            return Err(LedgerError::ReadOnlyAccount);
        }
        if source.lamports < lamports {
            return Err(LedgerError::InsufficientBalance);
        }

        source.lamports = source
            .lamports
            .checked_sub(lamports)
            .ok_or(LedgerError::ArithmeticBoundary)?;
        destination.lamports = destination
            .lamports
            .checked_add(lamports)
            .ok_or(LedgerError::ArithmeticBoundary)?;
        Ok(())
    }

    fn register_operator(
        program_id: &Address,
        accounts: &mut [SimAccount],
        callsign: String,
        tier: u8,
    ) -> Result<(), LedgerError> {
        if accounts.len() < 2 {
            return Err(LedgerError::MissingAccounts);
        }
        if callsign.len() > 32 {
            return Err(LedgerError::FieldTooLong);
        }

        let (entry_acc, rest) = accounts.split_at_mut(1);
        let entry_acc = &mut entry_acc[0];
        let authority = &rest[0];

        if !authority.is_signer {
            return Err(LedgerError::MissingSignature);
        }
        if &entry_acc.owner != program_id {
            return Err(LedgerError::OwnerMismatch);
        }

        let entry = OperatorRegistryEntry {
            initialized: true,
            authority: authority.address,
            callsign,
            tier,
        };
        let encoded = borsh::to_vec(&entry).map_err(|_| LedgerError::MalformedPayload)?;
        if encoded.len() > entry_acc.data.len() {
            entry_acc.data.resize(encoded.len(), 0);
        }
        entry_acc.data[..encoded.len()].copy_from_slice(&encoded);
        Ok(())
    }
}
