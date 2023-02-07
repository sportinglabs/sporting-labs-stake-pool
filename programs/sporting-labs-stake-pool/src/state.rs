use anchor_lang::prelude::*;

pub const TREASURY_PREFIX: &str = "treasury";
pub const TREASURY_SIZE: usize = 8 + std::mem::size_of::<StakeEntry>() + 8;

pub const STAKE_ENTRY_PREFIX: &str = "stake-entry";
pub const STAKE_ENTRY_SIZE: usize = 8 + std::mem::size_of::<StakeEntry>() + 8;

pub const STAKE_POOL_PREFIX: &str = "stake-pool";
// 5 Pubkeys for creators and collections
pub const STAKE_POOL_SIZE: usize = 8 + 1 + 8 + 32 + 4 + 3 * 32 + 4 + 1;

#[account]
pub struct Treasury {
    pub bump: u8,
    pub reward_mint: Pubkey,
    pub pool_count: u64,
}

#[account]
pub struct StakeEntry {
    pub bump: u8,
    pub pool: Pubkey,
    pub amount: u64,
    pub original_mint: Pubkey,
    pub original_mint_claimed: bool,
    pub last_staker: Pubkey,
}

pub enum PoolState {
    PreRace = 1,
    ActiveRace = 2,
    PostRace = 3,
}

pub enum RaceResult {
    Wet = 1,
    Dry = 2,
}

#[account()]
pub struct StakePool {
    pub bump: u8,
    pub identifier: u64,
    pub authority: Pubkey,
    pub requires_creators: Vec<Pubkey>,
    pub total_staked: u32,
    pub pool_state: u8,
    pub result: u8,
    // pub vrf: Pubkey,
    pub reward_mint: Pubkey,
}

pub fn stake_entry_fill_zeros<'info>(stake_entry: &mut Account<StakeEntry>) -> Result<()> {
    let stake_entry_account = stake_entry.to_account_info();
    let mut stake_entry_data = stake_entry_account.data.borrow_mut();
    let len = stake_entry_data.len();
    stake_entry_data[stake_entry.try_to_vec()?.len()..len].iter_mut().for_each(|d| *d = 0);
    Ok(())
}
