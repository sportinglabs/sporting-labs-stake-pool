use {
    crate::{state::*, errors::ErrorCode},
    anchor_lang::prelude::*,
    switchboard_v2::VrfAccountData
};

#[derive(Accounts)]
#[instruction(params: ConsumeRandomnessParams)] // rpc parameters hint
pub struct ConsumeRandomness<'info> {
    #[account(
        mut,
        seeds = [STAKE_POOL_PREFIX.as_bytes(), params.identifier.to_le_bytes().as_ref()],
        bump = params.bump,
        has_one = vrf @ ErrorCode::InvalidVrfAccount
    )]
    pub stake_pool: Account<'info, StakePool>,
    pub vrf: AccountLoader<'info, VrfAccountData>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ConsumeRandomnessParams {
    bump: u8,
    identifier: u32
}

impl ConsumeRandomness<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &ConsumeRandomnessParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: &ConsumeRandomnessParams) -> Result<()> {
        let vrf = ctx.accounts.vrf.load()?;
        let result_buffer = vrf.get_result()?;
        if result_buffer == [0u8; 32] {
            msg!("vrf buffer empty");
            return Ok(());
        }

        let stake_pool = &mut ctx.accounts.stake_pool;

        msg!("Result buffer is {:?}", result_buffer);
        let value: &[u8] = bytemuck::cast_slice(&result_buffer[..]);
        msg!("u128 buffer {:?}", value);
        let result = value[0] % 2 as u8 + 1;
        msg!("Current VRF Value [1 - 2) = {}!", result);

        if stake_pool.result != result {            
            stake_pool.result = result;
        }

        Ok(())
    }
}
