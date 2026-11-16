use anchor_lang::prelude::*;

/// Per-participant ledger record inside a ReservePool.
/// PDA seeds: `[b"member_ledger", pool.key().as_ref(), participant.key().as_ref()]`
#[account]
#[derive(Default)]
pub struct MemberLedger {
    pub pool: Pubkey,
    pub participant: Pubkey,
    pub net_contributed: u64,
    pub receipt_balance: u64,
    pub last_action_ts: i64,
    pub bump: u8,
}

impl MemberLedger {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1;
}
