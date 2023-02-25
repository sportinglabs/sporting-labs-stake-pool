use {crate::{state::*, errors::ErrorCode}, anchor_lang::prelude::*};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdatePoolIx {
    requires_creators: Vec<Pubkey>,
    authority: Pubkey,
    pool_state: u8,
    requires_active_traits: u16,
}

#[derive(Accounts)]
#[instruction(ix: UpdatePoolIx)]
pub struct UpdatePoolCtx<'info> {
    #[account(mut, constraint = stake_pool.authority == payer.key()
    @ ErrorCode::InvalidPoolAuthority)]
    stake_pool: Account<'info, StakePool>,

    #[account(mut)]
    payer: Signer<'info>,
}

pub fn handler(ctx: Context<UpdatePoolCtx>, ix: UpdatePoolIx) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;
    stake_pool.requires_creators = ix.requires_creators;
    stake_pool.authority = ix.authority;
    stake_pool.pool_state = ix.pool_state as u8;
    stake_pool.requires_active_traits = ix.requires_active_traits;

    // zero extra data
    let stake_pool_account = stake_pool.to_account_info();
    let mut stake_pool_data = stake_pool_account.data.borrow_mut();
    let len = stake_pool_data.len();
    stake_pool_data[stake_pool.try_to_vec()?.len()..len].iter_mut().for_each(|d| *d = 0);
    Ok(())
}
