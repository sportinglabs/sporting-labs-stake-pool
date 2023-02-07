use {
    crate::state::*,
    anchor_lang::prelude::*,
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
        seeds = [STAKE_POOL_PREFIX.as_bytes(), treasury.pool_count.to_le_bytes().as_ref()],
        bump
    )]
    stake_pool: Account<'info, StakePool>,
    #[account(mut)]
    treasury: Account<'info, Treasury>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitPoolCtx>, ix: InitPoolIx) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;
    let treasury = &mut ctx.accounts.treasury;
    stake_pool.bump = *ctx.bumps.get("stake_pool").unwrap();
    stake_pool.identifier = treasury.pool_count;
    stake_pool.requires_creators = ix.requires_creators;
    stake_pool.authority = ix.authority;
    stake_pool.total_staked = 0;
    stake_pool.pool_state = PoolState::PreRace as u8;
    stake_pool.result = 0;
    stake_pool.reward_mint = treasury.reward_mint;

    treasury.pool_count += 1;
    Ok(())
}
