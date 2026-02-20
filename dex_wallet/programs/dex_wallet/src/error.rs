use anchor_lang::prelude::*;

#[error_code]
pub enum DexError {
    #[msg("Invalid fee: must be between 0 and 1000 bps (10%)")]
    InvalidFee,

    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,

    #[msg("Insufficient liquidity in pool")]
    InsufficientLiquidity,

    #[msg("Invalid amount: must be greater than 0")]
    InvalidAmount,

    #[msg("Insufficient LP tokens to remove")]
    InsufficientLPTokens,

    #[msg("Math overflow")]
    MathOverflow,
}
