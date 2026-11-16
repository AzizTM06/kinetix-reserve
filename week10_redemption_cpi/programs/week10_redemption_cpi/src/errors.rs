use anchor_lang::prelude::*;

#[error_code]
pub enum ReserveError {
    #[msg("Amount must be strictly greater than zero")]
    ZeroAmount,
    #[msg("Exit fee cannot exceed 500 basis points (5%)")]
    FeeTooHigh,
    #[msg("Pool operations are temporarily paused by curator")]
    PoolPaused,
    #[msg("Contribution yields zero receipt tokens")]
    ReceiptsTooSmall,
    #[msg("Redemption yields zero underlying tokens")]
    RedemptionTooSmall,
    #[msg("Reserve pool has no issued receipts")]
    PoolEmpty,
    #[msg("Arithmetic overflow or division error")]
    MathOverflow,
    #[msg("Insufficient receipt balance on member ledger")]
    InsufficientReceipts,
    #[msg("Action restricted to pool curator")]
    Unauthorized,
    #[msg("Lower tick must be strictly less than upper tick")]
    InvalidTickRange,
    #[msg("Tick boundary is not aligned with pool tick_spacing")]
    UnalignedTick,
}
