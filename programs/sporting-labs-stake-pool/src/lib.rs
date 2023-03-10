pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use {
    anchor_lang::prelude::*,
    instructions::*,
};

declare_id!("D157bctkZhtTUqqESJ8QVUcVJHaGqTz1zaKSQPnxwdLc");

#[program]
pub mod sporting_labs_stake_pool {
    use super::*;

    pub fn init_treasury(ctx: Context<InitTreasuryCtx>) -> Result<()> {
        init_treasury::handler(ctx)
    }

    pub fn init_pool(ctx: Context<InitPoolCtx>, ix: InitPoolIx) -> Result<()> {
        init_pool::handler(ctx, ix)
    }

    pub fn init_entry(ctx: Context<InitEntryCtx>, user: Pubkey) -> Result<()> {
        init_entry::handler(ctx, user)
    }

    pub fn stake(ctx: Context<StakeCtx>, amount: u64) -> Result<()> {
        stake::handler(ctx, amount)
    }

    pub fn claim_receipt_mint<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, ClaimReceiptMintCtx<'info>>) -> Result<()> {
        claim_receipt_mint::handler(ctx)
    }

    pub fn unstake(ctx: Context<UnstakeCtx>) -> Result<()> {
        unstake::handler(ctx)
    }

    pub fn update_pool(ctx: Context<UpdatePoolCtx>, ix: UpdatePoolIx) -> Result<()> {
        update_pool::handler(ctx, ix)
    }

    pub fn return_receipt_mint<'key, 'accounts, 'remaining, 'info>(ctx: Context<'key, 'accounts, 'remaining, 'info, ReturnReceiptMintCtx<'info>>) -> Result<()> {
        return_receipt_mint::handler(ctx)
    }

    pub fn close_stake_pool(ctx: Context<CloseStakePoolCtx>) -> Result<()> {
        close_stake_pool::handler(ctx)
    }

    pub fn close_stake_entry(ctx: Context<CloseStakeEntryCtx>) -> Result<()> {
        close_stake_entry::handler(ctx)
    }

    pub fn stake_entry_fill_zeros(ctx: Context<StakeEntryFillZeros>) -> Result<()> {
        stake_entry_fill_zeros::handler(ctx)
    }

    pub fn stake_pool_fill_zeros(ctx: Context<StakePoolFillZeros>) -> Result<()> {
        stake_pool_fill_zeros::handler(ctx)
    }

    // #[access_control(ctx.accounts.validate(&ctx, &params))]
    // pub fn request_randomness(
    //     ctx: Context<RequestRandomness>,
    //     params: RequestRandomnessParams,
    // ) -> Result<()> {
    //     RequestRandomness::actuate(&ctx, &params)
    // }

    // #[access_control(ctx.accounts.validate(&ctx, &params))]
    // pub fn consume_randomness(
    //     ctx: Context<ConsumeRandomness>,
    //     params: ConsumeRandomnessParams,
    // ) -> Result<()> {
    //     ConsumeRandomness::actuate(ctx, &params)
    // }
}
