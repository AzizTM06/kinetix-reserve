use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub const MAX_TAG_LEN: usize = 48;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct BeaconRecord {
    pub sequence: u64,
    pub status_code: u16,
    pub tag: String,
}

impl BeaconRecord {
    pub const MAX_BYTES: usize = 8 + 2 + 4 + MAX_TAG_LEN;
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum BeaconInstruction {
    Ping { status_code: u16, tag: String },
    Reset,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let beacon_acc = next_account_info(iter)?;

    if beacon_acc.owner != program_id {
        msg!("Beacon account owner mismatch: expected {}", program_id);
        return Err(ProgramError::IncorrectProgramId);
    }
    if !beacon_acc.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    let ix = BeaconInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let mut data_ref = beacon_acc.data.borrow_mut();
    let mut cursor: &[u8] = &data_ref;
    let mut record = BeaconRecord::deserialize(&mut cursor).unwrap_or(BeaconRecord {
        sequence: 0,
        status_code: 0,
        tag: String::new(),
    });

    match ix {
        BeaconInstruction::Ping { status_code, tag } => {
            if tag.len() > MAX_TAG_LEN {
                return Err(ProgramError::InvalidInstructionData);
            }
            record.sequence = record
                .sequence
                .checked_add(1)
                .ok_or(ProgramError::ArithmeticOverflow)?;
            record.status_code = status_code;
            record.tag = tag;
            msg!(
                "Beacon ping #{} recorded (code={}, tag={})",
                record.sequence,
                record.status_code,
                record.tag
            );
        }
        BeaconInstruction::Reset => {
            record.sequence = 0;
            record.status_code = 0;
            record.tag = "reset".to_string();
            msg!("Beacon record cleared");
        }
    }

    let mut writer: &mut [u8] = &mut data_ref;
    record.serialize(&mut writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_ping_and_reset_roundtrip() {
        let program_id = Pubkey::new_unique();
        let key = Pubkey::new_unique();
        let mut lamports = 1_000_000u64;
        let mut data = vec![0u8; BeaconRecord::MAX_BYTES];

        let acc = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            Epoch::default(),
        );

        let ix_bytes = borsh::to_vec(&BeaconInstruction::Ping {
            status_code: 200,
            tag: "kinetix-heartbeat".to_string(),
        })
        .unwrap();

        process_instruction(&program_id, &[acc.clone()], &ix_bytes).unwrap();
        process_instruction(&program_id, &[acc.clone()], &ix_bytes).unwrap();

        let borrowed = acc.data.borrow();
        let mut cur1: &[u8] = &borrowed;
        let decoded = BeaconRecord::deserialize(&mut cur1).unwrap();
        assert_eq!(decoded.sequence, 2);
        assert_eq!(decoded.status_code, 200);
        assert_eq!(decoded.tag, "kinetix-heartbeat");
        drop(borrowed);

        let reset_bytes = borsh::to_vec(&BeaconInstruction::Reset).unwrap();
        process_instruction(&program_id, &[acc.clone()], &reset_bytes).unwrap();
        let borrowed2 = acc.data.borrow();
        let mut cur2: &[u8] = &borrowed2;
        let after_reset = BeaconRecord::deserialize(&mut cur2).unwrap();
        assert_eq!(after_reset.sequence, 0);
        assert_eq!(after_reset.tag, "reset");
    }

    #[test]
    fn test_rejects_foreign_owner() {
        let program_id = Pubkey::new_unique();
        let foreign_owner = Pubkey::new_unique();
        let key = Pubkey::new_unique();
        let mut lamports = 1_000_000u64;
        let mut data = vec![0u8; BeaconRecord::MAX_BYTES];

        let acc = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &foreign_owner,
            false,
            Epoch::default(),
        );

        let ix_bytes = borsh::to_vec(&BeaconInstruction::Reset).unwrap();
        let err = process_instruction(&program_id, &[acc], &ix_bytes).unwrap_err();
        assert_eq!(err, ProgramError::IncorrectProgramId);
    }
}
