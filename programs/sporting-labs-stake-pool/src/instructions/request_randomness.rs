use {
    crate::{state::*, errors::ErrorCode},
    anchor_lang::prelude::*,
    switchboard_v2::{OracleQueueAccountData, PermissionAccountData, SbState, VrfAccountData, VrfRequestRandomness},
    anchor_spl::token::{Token, TokenAccount}
};

#[derive(Accounts)]
#[instruction(params: RequestRandomnessParams)] // rpc parameters hint
pub struct RequestRandomness<'info> {
    #[account(
        mut,
        seeds = [STAKE_POOL_PREFIX.as_bytes(), params.identifier.to_le_bytes().as_ref()],
        bump = params.bump,
        has_one = vrf @ ErrorCode::InvalidVrfAccount
    )]
    pub stake_pool: Account<'info, StakePool>,

    // SWITCHBOARD ACCOUNTS
    #[account(mut,
        has_one = escrow
    )]
    pub vrf: AccountLoader<'info, VrfAccountData>,
    #[account(mut,
        has_one = data_buffer
    )]
    pub oracle_queue: AccountLoader<'info, OracleQueueAccountData>,
    /// CHECK:
    #[account(mut,
        constraint =
            oracle_queue.load()?.authority == queue_authority.key()
    )]
    pub queue_authority: UncheckedAccount<'info>,
    /// CHECK
    #[account(mut)]
    pub data_buffer: AccountInfo<'info>,
    #[account(mut)]
    pub permission: AccountLoader<'info, PermissionAccountData>,
    #[account(mut,
        constraint =
            escrow.owner == program_state.key()
            && escrow.mint == program_state.load()?.token_mint
    )]
    pub escrow: Account<'info, TokenAccount>,
    #[account(mut)]
    pub program_state: AccountLoader<'info, SbState>,
    /// CHECK:
    #[account(
        address = *vrf.to_account_info().owner,
        constraint = switchboard_program.executable == true
    )]
    pub switchboard_program: AccountInfo<'info>,

    // PAYER ACCOUNTS
    #[account(mut,
        constraint =
            payer_wallet.owner == payer_authority.key()
            && escrow.mint == program_state.load()?.token_mint
    )]
    pub payer_wallet: Account<'info, TokenAccount>,
    /// CHECK:
    #[account(signer)]
    pub payer_authority: AccountInfo<'info>,

    // SYSTEM ACCOUNTS
    /// CHECK:
    #[account(address = solana_program::sysvar::recent_blockhashes::ID)]
    pub recent_blockhashes: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct RequestRandomnessParams {
    pub permission_bump: u8,
    pub switchboard_state_bump: u8,
    pub identifier: u16,
    pub bump: u8
}

impl RequestRandomness<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &RequestRandomnessParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, params: &RequestRandomnessParams) -> Result<()> {
        let stake_pool = &ctx.accounts.stake_pool;
        let bump = stake_pool.bump.clone();

        let switchboard_program = ctx.accounts.switchboard_program.to_account_info();

        let vrf_request_randomness = VrfRequestRandomness {
            authority: ctx.accounts.stake_pool.to_account_info(),
            vrf: ctx.accounts.vrf.to_account_info(),
            oracle_queue: ctx.accounts.oracle_queue.to_account_info(),
            queue_authority: ctx.accounts.queue_authority.to_account_info(),
            data_buffer: ctx.accounts.data_buffer.to_account_info(),
            permission: ctx.accounts.permission.to_account_info(),
            escrow: ctx.accounts.escrow.clone(),
            payer_wallet: ctx.accounts.payer_wallet.clone(),
            payer_authority: ctx.accounts.payer_authority.to_account_info(),
            recent_blockhashes: ctx.accounts.recent_blockhashes.to_account_info(),
            program_state: ctx.accounts.program_state.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        let identifier_num = params.identifier.to_le_bytes();

        let stake_pool_seeds: &[&[&[u8]]] = &[&[&STAKE_POOL_PREFIX.as_bytes(), identifier_num.as_ref(), &[bump]]];

        msg!("requesting randomness");
        vrf_request_randomness.invoke_signed(
            switchboard_program,
            params.switchboard_state_bump,
            params.permission_bump,
            stake_pool_seeds,
        )?;

        msg!("randomness requested successfully");
        Ok(())
    }
}
