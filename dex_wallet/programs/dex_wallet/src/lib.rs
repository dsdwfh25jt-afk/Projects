use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("EKUTuQVm6JVNc7LwtuMjpCyRFBkQQgDFpsBMzXavfk1D");

#[program]
pub mod dex_wallet {
    use super::*;

    /// Initialize a new liquidity pool for token A and token B
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        fee_bps: u16,
    ) -> Result<()> {
        instructions::initialize_pool::handler(ctx, fee_bps)
    }

    /// Add liquidity to the pool
    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        instructions::add_liquidity::handler(ctx, amount_a, amount_b)
    }

    /// Remove liquidity from the pool
    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        lp_amount: u64,
    ) -> Result<()> {
        instructions::remove_liquidity::handler(ctx, lp_amount)
    }

    /// Swap token A for token B
    pub fn swap_a_for_b(
        ctx: Context<Swap>,
        amount_in: u64,
        min_amount_out: u64,
    ) -> Result<()> {
        instructions::swap::handler_swap_a_for_b(ctx, amount_in, min_amount_out)
    }

    /// Swap token B for token A
    pub fn swap_b_for_a(
        ctx: Context<Swap>,
        amount_in: u64,
        min_amount_out: u64,
    ) -> Result<()> {
        instructions::swap::handler_swap_b_for_a(ctx, amount_in, min_amount_out)
    }
}
