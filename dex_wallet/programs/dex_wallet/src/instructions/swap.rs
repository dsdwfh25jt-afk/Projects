use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};

use crate::constants;
use crate::error::DexError;
use crate::state::Pool;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::POOL_SEED],
        bump = pool.bump,
        has_one = vault_a,
        has_one = vault_b
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub user_token_in: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_out: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_b: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler_swap_a_for_b(
    ctx: Context<Swap>,
    amount_in: u64,
    min_amount_out: u64,
) -> Result<()> {
    require!(amount_in > 0, DexError::InvalidAmount);

    let pool = &ctx.accounts.pool;
    let fee_bps = pool.fee_bps as u64;
    let amount_in_after_fee = amount_in
        .checked_mul(10000 - fee_bps)
        .ok_or(DexError::MathOverflow)?
        .checked_div(10000)
        .ok_or(DexError::MathOverflow)?;

    let reserve_in = ctx.accounts.vault_a.amount;
    let reserve_out = ctx.accounts.vault_b.amount;
    require!(reserve_in > 0 && reserve_out > 0, DexError::InsufficientLiquidity);

    // Constant product: amount_out = (amount_in_after_fee * reserve_out) / (reserve_in + amount_in_after_fee)
    let amount_out = amount_in_after_fee
        .checked_mul(reserve_out)
        .ok_or(DexError::MathOverflow)?
        .checked_div(reserve_in.checked_add(amount_in_after_fee).ok_or(DexError::MathOverflow)?)
        .ok_or(DexError::MathOverflow)?;

    require!(amount_out >= min_amount_out, DexError::SlippageExceeded);

    let seeds = &[constants::POOL_SEED, &[pool.bump]];
    let signer_seeds = &[&seeds[..]];

    // User -> Vault A (token in)
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_in.to_account_info(),
                to: ctx.accounts.vault_a.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount_in,
    )?;

    // Vault B -> User (token out)
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_b.to_account_info(),
                to: ctx.accounts.user_token_out.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer_seeds,
        ),
        amount_out,
    )?;

    Ok(())
}

pub fn handler_swap_b_for_a(
    ctx: Context<Swap>,
    amount_in: u64,
    min_amount_out: u64,
) -> Result<()> {
    require!(amount_in > 0, DexError::InvalidAmount);

    let pool = &ctx.accounts.pool;
    let fee_bps = pool.fee_bps as u64;
    let amount_in_after_fee = amount_in
        .checked_mul(10000 - fee_bps)
        .ok_or(DexError::MathOverflow)?
        .checked_div(10000)
        .ok_or(DexError::MathOverflow)?;

    let reserve_in = ctx.accounts.vault_b.amount;
    let reserve_out = ctx.accounts.vault_a.amount;
    require!(reserve_in > 0 && reserve_out > 0, DexError::InsufficientLiquidity);

    let amount_out = amount_in_after_fee
        .checked_mul(reserve_out)
        .ok_or(DexError::MathOverflow)?
        .checked_div(reserve_in.checked_add(amount_in_after_fee).ok_or(DexError::MathOverflow)?)
        .ok_or(DexError::MathOverflow)?;

    require!(amount_out >= min_amount_out, DexError::SlippageExceeded);

    let seeds = &[constants::POOL_SEED, &[pool.bump]];
    let signer_seeds = &[&seeds[..]];

    // User -> Vault B (token in)
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_in.to_account_info(),
                to: ctx.accounts.vault_b.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount_in,
    )?;

    // Vault A -> User (token out)
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_a.to_account_info(),
                to: ctx.accounts.user_token_out.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer_seeds,
        ),
        amount_out,
    )?;

    Ok(())
}
