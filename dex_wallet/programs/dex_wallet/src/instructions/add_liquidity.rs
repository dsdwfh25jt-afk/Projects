use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, transfer};

use crate::constants;

fn isqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}
use crate::error::DexError;
use crate::state::Pool;

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
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
        constraint = user_lp_account.mint == lp_mint.key()
    )]
    pub user_lp_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
    require!(amount_a > 0 && amount_b > 0, DexError::InvalidAmount);

    let pool_bump = ctx.accounts.pool.bump;
    let total_lp_supply = ctx.accounts.pool.total_lp_supply;
    let reserve_a = ctx.accounts.vault_a.amount;
    let reserve_b = ctx.accounts.vault_b.amount;

    // Transfer token A to vault
    let cpi_accounts_a = Transfer {
        from: ctx.accounts.user_token_a.to_account_info(),
        to: ctx.accounts.vault_a.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts_a),
        amount_a,
    )?;

    // Transfer token B to vault
    let cpi_accounts_b = Transfer {
        from: ctx.accounts.user_token_b.to_account_info(),
        to: ctx.accounts.vault_b.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts_b),
        amount_b,
    )?;

    // Calculate LP tokens to mint (simple geometric mean for initial liquidity)
    let lp_amount = if total_lp_supply == 0 {
        let product = amount_a
            .checked_mul(amount_b)
            .ok_or(DexError::MathOverflow)?;
        isqrt(product)
    } else {
        require!(reserve_a > 0 && reserve_b > 0, DexError::InsufficientLiquidity);
        let lp_from_a = total_lp_supply
            .checked_mul(amount_a)
            .ok_or(DexError::MathOverflow)?
            .checked_div(reserve_a)
            .ok_or(DexError::MathOverflow)?;
        let lp_from_b = total_lp_supply
            .checked_mul(amount_b)
            .ok_or(DexError::MathOverflow)?
            .checked_div(reserve_b)
            .ok_or(DexError::MathOverflow)?;
        lp_from_a.min(lp_from_b)
    };

    require!(lp_amount > 0, DexError::InvalidAmount);

    // Mint LP tokens to user
    let seeds = &[constants::POOL_SEED, &[pool_bump]];
    let signer_seeds = &[&seeds[..]];

    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.lp_mint.to_account_info(),
                to: ctx.accounts.user_lp_account.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer_seeds,
        ),
        lp_amount,
    )?;

    let pool = &mut ctx.accounts.pool;
    pool.total_lp_supply = total_lp_supply
        .checked_add(lp_amount)
        .ok_or(DexError::MathOverflow)?;

    Ok(())
}
