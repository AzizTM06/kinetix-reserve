use anchor_lang::prelude::*;

pub const VIRTUAL_OFFSET: u128 = 1_000;
pub const BPS_DENOMINATOR: u128 = 10_000;
pub const MAX_FEE_BPS: u16 = 500;

/// Primary reserve pool state.
/// PDA seeds: `[b"kinetix_pool", base_mint.key().as_ref()]`
#[account]
#[derive(Default)]
pub struct ReservePool {
    pub curator: Pubkey,
    pub base_mint: Pubkey,
    pub reserve_account: Pubkey,
    pub receipt_mint: Pubkey,
    pub managed_assets: u64,
    pub issued_receipts: u64,
    pub fee_bps: u16,
    pub paused: bool,
    pub bump: u8,
}

impl ReservePool {
    pub const SIZE: usize = 8 + 32 + 32 + 32 + 32 + 8 + 8 + 2 + 1 + 1;

    pub fn compute_receipts_for_deposit(&self, amount: u64) -> Option<u64> {
        let num = (amount as u128).checked_mul((self.issued_receipts as u128).checked_add(VIRTUAL_OFFSET)?)?;
        let den = (self.managed_assets as u128).checked_add(VIRTUAL_OFFSET)?;
        u64::try_from(num.checked_div(den)?).ok()
    }

    pub fn compute_assets_for_redemption(&self, receipts: u64) -> Option<(u64, u64)> {
        let gross_num = (receipts as u128).checked_mul((self.managed_assets as u128).checked_add(VIRTUAL_OFFSET)?)?;
        let gross_den = (self.issued_receipts as u128).checked_add(VIRTUAL_OFFSET)?;
        let gross_u128 = gross_num.checked_div(gross_den)?;
        let fee_u128 = gross_u128.checked_mul(self.fee_bps as u128)?.checked_div(BPS_DENOMINATOR)?;
        let net_u128 = gross_u128.checked_sub(fee_u128)?;
        let net = u64::try_from(net_u128).ok()?;
        let fee = u64::try_from(fee_u128).ok()?;
        Some((net, fee))
    }
}
