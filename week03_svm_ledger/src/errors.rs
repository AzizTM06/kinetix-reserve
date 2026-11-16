use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    #[error("Missing required transaction signature")]
    MissingSignature,
    #[error("Account is not marked writable")]
    ReadOnlyAccount,
    #[error("Account owner mismatch with executing program")]
    OwnerMismatch,
    #[error("Insufficient lamports or quota remaining")]
    InsufficientBalance,
    #[error("Checked arithmetic overflow or underflow")]
    ArithmeticBoundary,
    #[error("Malformed Borsh instruction payload")]
    MalformedPayload,
    #[error("Not enough accounts supplied to instruction")]
    MissingAccounts,
    #[error("Quota exceeded daily epoch cap")]
    QuotaCapExceeded,
    #[error("String field exceeds allowed byte budget")]
    FieldTooLong,
}
