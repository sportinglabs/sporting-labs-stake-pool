use {
    crate::{state::*, errors::ErrorCode},
    anchor_lang::prelude::*,
    switchboard_v2::VrfAccountData,
    anchor_spl::token::{self, Token, TokenAccount},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitPoolIx {
    requires_creators: Vec<Pubkey>,
    authority: Pubkey,
    supply: u64,
}

#[derive(Accounts)]
#[instruction(ix: InitPoolIx)]
pub struct InitPoolCtx<'info> {
    #[account(
        init,
        payer = payer,
        space = STAKE_POOL_SIZE,
        seeds = [STAKE_POOL_PREFIX.as_bytes(), identifier.count.to_le_bytes().as_ref()],
        bump
    )]
    stake_pool: Account<'info, StakePool>,
    #[account(mut)]
    identifier: Account<'info, Identifier>,
    // Reward Mint
    stake_pool_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    authority_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = vrf.load()?.authority == stake_pool.key() @ ErrorCode::InvalidVrfAuthorityError
    )]
    vrf: AccountLoader<'info, VrfAccountData>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<InitPoolCtx>, ix: InitPoolIx) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;
    let identifier = &mut ctx.accounts.identifier;
    stake_pool.bump = *ctx.bumps.get("stake_pool").unwrap();
    stake_pool.identifier = identifier.count;
    stake_pool.requires_creators = ix.requires_creators;
    stake_pool.authority = ix.authority;
    stake_pool.total_staked = 0;
    stake_pool.pool_state = PoolState::PreRace as u8;
    stake_pool.result = 0;
    stake_pool.vrf = ctx.accounts.vrf.key();

    let identifier = &mut ctx.accounts.identifier;
    identifier.count += 1;

    // Transfer reward token
    let authority_token_account = &mut ctx.accounts.authority_token_account;
    let reward_distributor_token_account = &mut ctx.accounts.stake_pool_token_account;

    let cpi_accounts = token::Transfer {
        from: authority_token_account.to_account_info(),
        to: reward_distributor_token_account.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_context, ix.supply)?;      
    Ok(())
}
