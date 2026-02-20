use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants;
use crate::error::DexError;
use crate::state::Pool;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Pool::LEN,
        seeds = [constants::POOL_SEED],
        bump
    )]
    pub pool: Account<'info, Pool>,

    pub token_mint_a: Account<'info, Mint>,
    pub token_mint_b: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        seeds = [constants::VAULT_A_SEED],
        bump,
        token::mint = token_mint_a,
        token::authority = pool
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        seeds = [constants::VAULT_B_SEED],
        bump,
        token::mint = token_mint_b,
        token::authority = pool
    )]
    pub vault_b: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        seeds = [constants::LP_MINT_SEED],
        bump,
        mint::decimals = 6,
        mint::authority = pool
    )]
    pub lp_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<InitializePool>, fee_bps: u16) -> Result<()> {
    require!(fee_bps <= constants::MAX_FEE_BPS, DexError::InvalidFee);

    let pool = &mut ctx.accounts.pool;
    pool.token_mint_a = ctx.accounts.token_mint_a.key();
    pool.token_mint_b = ctx.accounts.token_mint_b.key();
    pool.vault_a = ctx.accounts.vault_a.key();
    pool.vault_b = ctx.accounts.vault_b.key();
    pool.lp_mint = ctx.accounts.lp_mint.key();
    pool.fee_bps = fee_bps;
    pool.total_lp_supply = 0;
    pool.bump = ctx.bumps.pool;
    pool.authority_bump = ctx.bumps.pool;

    Ok(())
}
