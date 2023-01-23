use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid user original mint token account")]
    InvalidUserOriginalMintTokenAccount,
    #[msg("Invalid stake entry original mint token account")]
    InvalidStakeEntryOriginalMintTokenAccount,
    #[msg("Invalid stake entry token manager mint token account")]
    InvalidStakeEntryMintTokenAccount,
    #[msg("Invalid unstake user only last staker can unstake")]
    InvalidUnstakeUser,
    #[msg("Invalid stake pool")]
    InvalidStakePool,
    #[msg("Mint not allowed in this pool")]
    MintNotAllowedInPool,
    #[msg("Invalid stake pool authority")]
    InvalidPoolAuthority,
    #[msg("Invalid stake type")]
    InvalidStakeType,
    #[msg("Invalid last staker")]
    InvalidLastStaker,
    #[msg("Invalid receipt mint")]
    InvalidReceiptMint,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Cannot close staked entry")]
    CannotCloseStakedEntry,
    #[msg("Cannot close staked entry")]
    CannotClosePoolWithStakedEntries,
    #[msg("Invalid mint metadata")]
    InvalidMintMetadata,
    #[msg("Staking Phase has ended")]
    StakingPhaseHasEnded,
    #[msg("Race is ongoing")]
    RaceIsOngoing,
    #[msg("Mint metadata is owned by the incorrect program")]
    InvalidMintMetadataOwner,
    #[msg("Invalid stake entry")]
    InvalidStakeEntry,
    #[msg("Invalid Vrf Authority Error")]
    InvalidVrfAuthorityError,
    #[msg("Invalid Vrf Account")]
    InvalidVrfAccount,
    #[msg("No Result")]
    NoResult,
    #[msg("Invalid Reward Mint")]
    InvalidRewardMint,
    #[msg("Invalid User Reward Mint Token Account")]
    InvalidUserRewardMintTokenAccount,
}
