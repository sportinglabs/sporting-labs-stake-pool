use {
    crate::{state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{self, Token, Mint, SetAuthority},
    spl_token::instruction::AuthorityType
};

#[derive(Accounts)]
pub struct InitTreasuryCtx<'info> {
    #[account(
        init,
        payer = payer,
        space = TREASURY_SIZE,
        seeds = [TREASURY_PREFIX.as_bytes()],
        bump
    )]
    treasury: Account<'info, Treasury>,
    #[account(mut)]
    reward_mint: Account<'info, Mint>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<InitTreasuryCtx>) -> Result<()> {

  let treasury = &mut ctx.accounts.treasury;
  treasury.pool_count = 1;
    
  let cpi_accounts = SetAuthority {
    account_or_mint: ctx.accounts.reward_mint.to_account_info(),
    current_authority: ctx.accounts.payer.to_account_info(),
  };

  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
  token::set_authority(cpi_context, AuthorityType::MintTokens, Some(treasury.key()))?;
      
  Ok(())
}
