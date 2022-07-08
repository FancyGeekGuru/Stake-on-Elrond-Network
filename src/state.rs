elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct StakeAccount<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub staked_amount: BigUint<M>,
    pub lock_end_timestamp: u64,
    pub unstaked_amount: BigUint<M>,
    pub undelegation_end_timestamp: u64,
    pub collectable_amount: BigUint<M>,
    pub reward_amount: BigUint<M>,
    pub last_reward_updated_timestamp: u64,
    pub last_claim_timestamp: u64,
}

#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct RewardApr {
    pub id: usize,
    pub apr: u32,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
}

#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct StakeSetting<M: ManagedTypeApi> {
    pub stake_token_id: TokenIdentifier<M>,
    pub reward_token_id: TokenIdentifier<M>,
    pub min_stake_limit: BigUint<M>,
    pub max_stake_limit: BigUint<M>, // 0 for no limit
    pub lock_period: u64,
    pub undelegation_period: u64,
    pub claim_lock_period: u64,
    pub stake_token_price: BigUint<M>,
    pub reward_token_price: BigUint<M>,

    pub apr: u32,
    pub total_staked_amount: BigUint<M>,
    pub number_of_stakers: u32,
}