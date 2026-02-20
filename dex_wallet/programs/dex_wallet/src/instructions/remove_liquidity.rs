use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, Mint, Token, TokenAccount, Transfer, burn};

use crate::constants;
use crate::error::DexError;
use crate::state::Pool;

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::POOL_SEED],
        bump = pool.bump,
        has_one = vault_a,
        has_one = vault_b,
        has_one = lp_mint
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = user_lp_account.mint == lp_mint.key(),
        constraint = user_lp_account.owner == user.key()
    )]
    pub user_lp_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<RemoveLiquidity>, lp_amount: u64) -> Result<()> {
    require!(lp_amount > 0, DexError::InvalidAmount);

    let pool_bump = ctx.accounts.pool.bump;
    let supply = ctx.accounts.pool.total_lp_supply;
    require!(
        ctx.accounts.user_lp_account.amount >= lp_amount,
        DexError::InsufficientLPTokens
    );
    require!(supply > 0, DexError::InsufficientLiquidity);

    let vault_a_balance = ctx.accounts.vault_a.amount;
    let vault_b_balance = ctx.accounts.vault_b.amount;

    let amount_a_out = vault_a_balance
        .checked_mul(lp_amount)
        .ok_or(DexError::MathOverflow)?
        .checked_div(supply)
        .ok_or(DexError::MathOverflow)?;
    let amount_b_out = vault_b_balance
        .checked_mul(lp_amount)
        .ok_or(DexError::MathOverflow)?
        .checked_div(supply)
        .ok_or(DexError::MathOverflow)?;

    // Burn LP tokens
    let seeds = &[constants::POOL_SEED, &[pool_bump]];
    let signer_seeds = &[&seeds[..]];

    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.lp_mint.to_account_info(),
                from: ctx.accounts.user_lp_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        lp_amount,
    )?;

    // Transfer tokens from vault to user
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_a.to_account_info(),
                to: ctx.accounts.user_token_a.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer_seeds,
        ),
        amount_a_out,
    )?;

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_b.to_account_info(),
                to: ctx.accounts.user_token_b.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer_seeds,
        ),
        amount_b_out,
    )?;

    let pool = &mut ctx.accounts.pool;
    pool.total_lp_supply = supply
        .checked_sub(lp_amount)
        .ok_or(DexError::MathOverflow)?;

    Ok(())
}
