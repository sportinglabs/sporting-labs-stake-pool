use {
    crate::{state::*, errors::ErrorCode},
    anchor_lang::prelude::*,
    switchboard_v2::VrfAccountData
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitPoolIx {
    requires_creators: Vec<Pubkey>,
    authority: Pubkey,
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
    #[account(
        constraint = vrf.load()?.authority == stake_pool.key() @ ErrorCode::InvalidVrfAuthorityError
    )]
    vrf: AccountLoader<'info, VrfAccountData>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
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
    Ok(())
}
