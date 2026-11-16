mod engine;
mod errors;
mod instructions;
mod ledger;

use borsh::BorshDeserialize;
use engine::LedgerEngine;
use errors::LedgerError;
use instructions::LedgerInstruction;
use ledger::{Address, EpochQuotaState, OperatorRegistryEntry, SimAccount};

fn run_simulation() {
    let program_id = Address::from_seed(0xAA);
    let operator = SimAccount::new(Address::from_seed(0x01), Address::from_seed(0x00), 5_000_000, 0, true, true);
    let treasurer = SimAccount::new(Address::from_seed(0x02), Address::from_seed(0x00), 1_000_000, 0, false, true);
    let quota_pda = SimAccount::new(Address::from_seed(0x10), program_id, 0, 128, false, true);
    let registry_pda = SimAccount::new(Address::from_seed(0x11), program_id, 0, 128, false, true);

    let mut quota_accounts = vec![quota_pda, operator.clone()];
    let init_ix = LedgerInstruction::InitQuota { epoch: 42, daily_cap: 100_000 }.encode().unwrap();
    LedgerEngine::execute(&program_id, &mut quota_accounts, &init_ix).unwrap();

    let consume_ix = LedgerInstruction::ConsumeQuota { amount: 35_000 }.encode().unwrap();
    LedgerEngine::execute(&program_id, &mut quota_accounts, &consume_ix).unwrap();

    let mut cur: &[u8] = &quota_accounts[0].data;
    let state = EpochQuotaState::deserialize(&mut cur).unwrap();
    println!("[Quota] epoch={}, consumed={}/{}, tx_count={}", state.epoch, state.consumed, state.daily_cap, state.settlements_count);

    let mut transfer_accounts = vec![operator.clone(), treasurer];
    let settle_ix = LedgerInstruction::SettleLamports { lamports: 750_000 }.encode().unwrap();
    LedgerEngine::execute(&program_id, &mut transfer_accounts, &settle_ix).unwrap();
    println!("[Settlement] operator={} lamports, treasurer={} lamports", transfer_accounts[0].lamports, transfer_accounts[1].lamports);

    let mut reg_accounts = vec![registry_pda, operator];
    let reg_ix = LedgerInstruction::RegisterOperator { callsign: "kinetix-node-01".into(), tier: 2 }.encode().unwrap();
    LedgerEngine::execute(&program_id, &mut reg_accounts, &reg_ix).unwrap();
    let mut reg_cur: &[u8] = &reg_accounts[0].data;
    let entry = OperatorRegistryEntry::deserialize(&mut reg_cur).unwrap();
    println!("[Registry] callsign={}, tier={}", entry.callsign, entry.tier);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("simulate") | None => run_simulation(),
        Some("quota") => {
            let cap: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(50_000);
            let debit: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(12_500);
            let program_id = Address::from_seed(0xAA);
            let op = SimAccount::new(Address::from_seed(0x01), Address::from_seed(0x00), 1_000_000, 0, true, true);
            let pda = SimAccount::new(Address::from_seed(0x10), program_id, 0, 128, false, true);
            let mut accs = vec![pda, op];
            LedgerEngine::execute(&program_id, &mut accs, &LedgerInstruction::InitQuota { epoch: 1, daily_cap: cap }.encode().unwrap()).unwrap();
            LedgerEngine::execute(&program_id, &mut accs, &LedgerInstruction::ConsumeQuota { amount: debit }.encode().unwrap()).unwrap();
            let mut cur: &[u8] = &accs[0].data;
            let st = EpochQuotaState::deserialize(&mut cur).unwrap();
            println!("Quota updated: {}/{}", st.consumed, st.daily_cap);
        }
        Some("fault-check") => {
            let program_id = Address::from_seed(0xAA);
            let unsigned = SimAccount::new(Address::from_seed(0x01), Address::from_seed(0x00), 100, 0, false, true);
            let dst = SimAccount::new(Address::from_seed(0x02), Address::from_seed(0x00), 100, 0, false, true);
            let mut accs = vec![unsigned, dst];
            let err = LedgerEngine::execute(&program_id, &mut accs, &LedgerInstruction::SettleLamports { lamports: 50 }.encode().unwrap());
            assert_eq!(err, Err(LedgerError::MissingSignature));
            println!("Fault check passed: {:?}", err.unwrap_err());
        }
        Some(other) => {
            eprintln!("Unknown command '{}'. Available: simulate, quota <cap> <debit>, fault-check", other);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_lifecycle_and_cap_guard() {
        let program_id = Address::from_seed(0xCC);
        let op = SimAccount::new(Address::from_seed(0x05), Address::from_seed(0x00), 10_000, 0, true, true);
        let pda = SimAccount::new(Address::from_seed(0x06), program_id, 0, 128, false, true);
        let mut accounts = vec![pda, op];

        LedgerEngine::execute(
            &program_id,
            &mut accounts,
            &LedgerInstruction::InitQuota { epoch: 7, daily_cap: 1_000 }.encode().unwrap(),
        ).unwrap();

        LedgerEngine::execute(
            &program_id,
            &mut accounts,
            &LedgerInstruction::ConsumeQuota { amount: 600 }.encode().unwrap(),
        ).unwrap();

        let err = LedgerEngine::execute(
            &program_id,
            &mut accounts,
            &LedgerInstruction::ConsumeQuota { amount: 500 }.encode().unwrap(),
        );
        assert_eq!(err, Err(LedgerError::QuotaCapExceeded));

        LedgerEngine::execute(
            &program_id,
            &mut accounts,
            &LedgerInstruction::ReplenishQuota { amount: 200 }.encode().unwrap(),
        ).unwrap();

        let mut cur: &[u8] = &accounts[0].data;
        let state = EpochQuotaState::deserialize(&mut cur).unwrap();
        assert_eq!(state.consumed, 400);
        assert_eq!(state.settlements_count, 1);
    }

    #[test]
    fn test_lamports_settlement_split_mut() {
        let program_id = Address::from_seed(0xCC);
        let a = SimAccount::new(Address::from_seed(0x01), Address::from_seed(0x00), 2_000, 0, true, true);
        let b = SimAccount::new(Address::from_seed(0x02), Address::from_seed(0x00), 500, 0, false, true);
        let mut accounts = vec![a, b];

        LedgerEngine::execute(
            &program_id,
            &mut accounts,
            &LedgerInstruction::SettleLamports { lamports: 1_250 }.encode().unwrap(),
        ).unwrap();

        assert_eq!(accounts[0].lamports, 750);
        assert_eq!(accounts[1].lamports, 1_750);
    }
}
