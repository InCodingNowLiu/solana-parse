use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UserRewardInfo {
    pub reward_per_token_stored: [u128; 2],
    pub reward_debt: [u128; 2],
    pub pending_reward: [u64; 2],
    pub _reserved: [u8; 32],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FeeInfo {
    pub fee_amount_x_per_token_stored: u128,
    pub fee_amount_y_per_token_stored: u128,
    pub pending_fee_x: u64,
    pub pending_fee_y: u64,
    pub _reserved: [u8; 32],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PositionV2 {
    pub lb_pair: Pubkey,
    pub owner: Pubkey,
    pub liquidity_shares: [u128; 70],
    pub reward_infos: [UserRewardInfo; 70],
    pub fee_infos: [FeeInfo; 70],
    pub lower_bin_id: i32,
    pub upper_bin_id: i32,
    pub last_updated_at: i64,
    pub total_claimed_fee_x_amount: u64,
    pub total_claimed_fee_y_amount: u64,
    pub total_claimed_rewards: [u64; 2],
    pub operator: Pubkey,
    pub lock_release_point: u64,
    pub _reserved: [u8; 160],
}
