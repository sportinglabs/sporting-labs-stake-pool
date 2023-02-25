use raindrops_player::{
    BasicStat, BasicStatState, PermissivenessType, PlayerData, UpdatePlayerArgs,
};

use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{self, Mint, Token, TokenAccount},
    raindrops_player::cpi::accounts::UpdatePlayer,
    raindrops_player::program::RaindropsPlayer,
    raindrops_player::{self, Player, PlayerClass},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UnstakeArgs {
    player_class_mint: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: UnstakeArgs)]
pub struct UnstakeCtx<'info> {
    #[account(mut)]
    stake_pool: Box<Account<'info, StakePool>>,
    treasury: Account<'info, Treasury>,
    #[account(mut, constraint = stake_entry.pool == stake_pool.key() @ ErrorCode::InvalidStakePool)]
    stake_entry: Box<Account<'info, StakeEntry>>,

    original_mint: Box<Account<'info, Mint>>,

    #[account(
      mut,
      seeds=[
          PLAYER_PREFIX.as_bytes(),
          args.player_class_mint.as_ref(),
          &12u64.to_le_bytes()
      ],
      bump=player_class.bump
  )]
    player_class: Box<Account<'info, PlayerClass>>,
    #[account(
      mut,
      constraint=player.parent == player_class.key() && player.mint.expect("Player has no mint") == original_mint.key(),
      seeds=[
          PLAYER_PREFIX.as_bytes(),
          original_mint.key().as_ref(),
          &12u64.to_le_bytes()
      ],
      bump=player.bump
    )]
    player: Box<Account<'info, Player>>,

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
    #[account(mut, constraint = reward_mint.key() == treasury.reward_mint @ ErrorCode::InvalidRewardMint)]
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
    raindrops_player: Program<'info, RaindropsPlayer>,
}

pub fn handler(ctx: Context<UnstakeCtx>) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;
    let treasury = &mut ctx.accounts.treasury;
    let stake_entry = &mut ctx.accounts.stake_entry;

    let bump = treasury.bump.clone();
    let treasury_authority_seeds: &[&[&[u8]]] = &[&[&TREASURY_PREFIX.as_bytes(), &[bump]]];

    let player = &mut ctx.accounts.player;
    for item in player.equipped_items.iter() {}

    // Update Points

    let cpi_program = ctx.accounts.raindrops_player.to_account_info();
    let args = UpdatePlayerArgs {
        class_index: 16,
        index: 16,
        player_mint: ctx.accounts.original_mint.key(),
        update_permissiveness_to_use: Some(PermissivenessType::ParentTokenHolder),
        player_class_mint: ctx.accounts.player_class.key(),
        new_data: Some(PlayerData {
            stats_uri: None,
            category: None,
            basic_stats: Some(vec![BasicStat {
                index: 0,
                state: BasicStatState::Integer {
                    base: (10),
                    with_temporary_changes: (0),
                    temporary_numerator: (0),
                    temporary_denominator: (0),
                    finalized: (0),
                },
            }]),
        }),
    };
    let cpi_accounts = UpdatePlayer {
        player: ctx.accounts.player.to_account_info(),
        player_class: ctx.accounts.player_class.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    raindrops_player::cpi::update_player(cpi_ctx, args)?;

    if stake_pool.pool_state == PoolState::ActiveRace as u8 {
        return Err(error!(ErrorCode::RaceIsOngoing));
    }

    if stake_pool.result != 0 {
        // TODO: Assert F1 car traits

        let mut reward_amount_to_receive: u64 = 0;

        let wheel_trait = RaceResult::Dry;

        if stake_pool.result == wheel_trait as u8 {
            reward_amount_to_receive += 20;
        }

        // Mint soulbound tokens
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.reward_mint.to_account_info(),
            to: ctx
                .accounts
                .user_reward_mint_token_account
                .to_account_info(),
            authority: treasury.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context =
            CpiContext::new(cpi_program, cpi_accounts).with_signer(treasury_authority_seeds);
        token::mint_to(
            cpi_context,
            reward_amount_to_receive
                .try_into()
                .expect("Too many rewards to receive"),
        )?;

        // Freeze token account
        let cpi_accounts = token::FreezeAccount {
            account: ctx
                .accounts
                .user_reward_mint_token_account
                .to_account_info(),
            mint: ctx.accounts.reward_mint.to_account_info(),
            authority: treasury.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context =
            CpiContext::new(cpi_program, cpi_accounts).with_signer(treasury_authority_seeds);
        token::freeze_account(cpi_context)?;
    }

    // Set up keys to return NFT
    let original_mint = stake_entry.original_mint;
    let user = ctx.accounts.user.key();
    let stake_pool_key = stake_pool.key();

    let stake_entry_seed = [
        STAKE_ENTRY_PREFIX.as_bytes(),
        stake_pool_key.as_ref(),
        original_mint.as_ref(),
        user.as_ref(),
        &[stake_entry.bump],
    ];
    let stake_entry_signer = &[&stake_entry_seed[..]];

    // give back original mint to user
    let cpi_accounts = token::Transfer {
        from: ctx
            .accounts
            .stake_entry_original_mint_token_account
            .to_account_info(),
        to: ctx
            .accounts
            .user_original_mint_token_account
            .to_account_info(),
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
