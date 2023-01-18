use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{self, Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct StakeCtx<'info> {
    #[account(mut, seeds = [STAKE_ENTRY_PREFIX.as_bytes(), stake_entry.pool.as_ref(), stake_entry.original_mint.as_ref(), user.key().as_ref()], bump=stake_entry.bump)]
    stake_entry: Box<Account<'info, StakeEntry>>,

    #[account(mut, constraint = stake_entry.pool == stake_pool.key() @ ErrorCode::InvalidStakePool)]
    stake_pool: Box<Account<'info, StakePool>>,

    // stake_entry token accounts
    #[account(mut, constraint =
        stake_entry_original_mint_token_account.mint == stake_entry.original_mint
        && stake_entry_original_mint_token_account.owner == stake_entry.key()
        @ ErrorCode::InvalidStakeEntryOriginalMintTokenAccount
    )]
    stake_entry_original_mint_token_account: Box<Account<'info, TokenAccount>>,
    original_mint: Box<Account<'info, Mint>>,

    // user
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut, constraint =
        user_original_mint_token_account.amount > 0
        && user_original_mint_token_account.mint == stake_entry.original_mint
        && user_original_mint_token_account.owner == user.key()
        @ ErrorCode::InvalidUserOriginalMintTokenAccount
    )]
    user_original_mint_token_account: Box<Account<'info, TokenAccount>>,

    // programs
    token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<StakeCtx>, amount: u64) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;
    let stake_entry = &mut ctx.accounts.stake_entry;

    if stake_pool.pool_state == PoolState::ActiveRace as u8 || stake_pool.pool_state == PoolState::PostRace as u8 {
        return Err(error!(ErrorCode::StakingPhaseHasEnded));
    }

    // transfer original
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.user_original_mint_token_account.to_account_info(),
        to: ctx.accounts.stake_entry_original_mint_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_context, amount)?;

    // update stake entry
    stake_entry.last_staker = ctx.accounts.user.key();
    stake_entry.amount = stake_entry.amount.checked_add(amount).unwrap();
    stake_pool.total_staked = stake_pool.total_staked.checked_add(1).expect("Add error");

    Ok(())
}
