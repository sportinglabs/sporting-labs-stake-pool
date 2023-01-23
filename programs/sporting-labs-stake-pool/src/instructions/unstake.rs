use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{self, Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct UnstakeCtx<'info> {
    #[account(mut)]
    stake_pool: Box<Account<'info, StakePool>>,
    treasury_authority: Account<'info, TreasuryAuthority>,
    #[account(mut, constraint = stake_entry.pool == stake_pool.key() @ ErrorCode::InvalidStakePool)]
    stake_entry: Box<Account<'info, StakeEntry>>,

    original_mint: Box<Account<'info, Mint>>,

    // stake_entry token account
    #[account(mut, constraint =
        (stake_entry_original_mint_token_account.amount > 0)
        && stake_entry_original_mint_token_account.mint == stake_entry.original_mint
        && stake_entry_original_mint_token_account.owner == stake_entry.key()
        @ ErrorCode::InvalidStakeEntryOriginalMintTokenAccount)]
    stake_entry_original_mint_token_account: Box<Account<'info, TokenAccount>>,

    // reward distribution
    #[account(mut, constraint = user_reward_mint_token_account.mint == stake_pool.reward_mint && user_reward_mint_token_account.owner == user.key() @ ErrorCode::InvalidUserRewardMintTokenAccount)]
    user_reward_mint_token_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = reward_mint.key() == treasury_authority.reward_mint @ ErrorCode::InvalidRewardMint)]
    reward_mint: Account<'info, Mint>,

    // user
    #[account(mut, constraint = user.key() == stake_entry.last_staker @ ErrorCode::InvalidUnstakeUser)]
    user: Signer<'info>,
    #[account(mut, constraint =
        user_original_mint_token_account.mint == stake_entry.original_mint
        && user_original_mint_token_account.owner == user.key()
        @ ErrorCode::InvalidUserOriginalMintTokenAccount)]
    user_original_mint_token_account: Box<Account<'info, TokenAccount>>,

    // programs
    token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<UnstakeCtx>) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;
    let treasury_authority = &mut ctx.accounts.treasury_authority;
    let stake_entry = &mut ctx.accounts.stake_entry;

    let bump = treasury_authority.bump.clone();
    let treasury_authority_seeds: &[&[&[u8]]] = &[&[&TREASURY_AUTHORITY_PREFIX.as_bytes(), &[bump]]];

    if stake_pool.pool_state == PoolState::ActiveRace as u8 {
        return Err(error!(ErrorCode::RaceIsOngoing));
    }

    if  stake_pool.result != 0 {
        // TODO: Assert F1 car traits

        let mut reward_amount_to_receive: u64 = 0;

        let wheel_trait = RaceResult::Dry;

        if stake_pool.result == wheel_trait as u8 {
            reward_amount_to_receive += 20;
        }

        // Mint soulbound tokens
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.reward_mint.to_account_info(),
            to: ctx.accounts.user_reward_mint_token_account.to_account_info(),
            authority: treasury_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(treasury_authority_seeds);
        token::mint_to(cpi_context, reward_amount_to_receive.try_into().expect("Too many rewards to receive"))?;
    }

    // Set up keys to return NFT
    let original_mint = stake_entry.original_mint;
    let user = ctx.accounts.user.key();
    let stake_pool_key = stake_pool.key();

    let stake_entry_seed = [STAKE_ENTRY_PREFIX.as_bytes(), stake_pool_key.as_ref(), original_mint.as_ref(), user.as_ref(), &[stake_entry.bump]];
    let stake_entry_signer = &[&stake_entry_seed[..]];

    // give back original mint to user
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.stake_entry_original_mint_token_account.to_account_info(),
        to: ctx.accounts.user_original_mint_token_account.to_account_info(),
        authority: stake_entry.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(stake_entry_signer);
    token::transfer(cpi_context, stake_entry.amount)?;

    stake_entry.last_staker = Pubkey::default();
    stake_entry.original_mint_claimed = false;
    stake_entry.amount = 0;
    stake_pool.total_staked = stake_pool.total_staked.checked_sub(1).expect("Sub error");
    stake_entry_fill_zeros(stake_entry)?;
    Ok(())
}
