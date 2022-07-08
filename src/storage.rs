elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::state:: {RewardApr};

#[elrond_wasm::module]
pub trait StorageModule {
    //
    #[view(getStakeTokenId)]
    #[storage_mapper("stake_token_id")]
    fn stake_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[only_owner]
    #[endpoint(setStakeTokenId)]
    fn set_stake_token_id(
        &self,
        stake_token_id: TokenIdentifier
    ) {
        self.stake_token_id().set(stake_token_id);
    }

    //
    #[view(getRewardTokenId)]
    #[storage_mapper("reward_token_id")]
    fn reward_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[only_owner]
    #[endpoint(setRewardTokenId)]
    fn set_reward_token_id(
        &self,
        reward_token_id: TokenIdentifier
    ) {
        self.reward_token_id().set(reward_token_id);
    }

    //
    #[view(getMinStakeLimit)]
    #[storage_mapper("min_stake_limit")]
    fn min_stake_limit(&self) -> SingleValueMapper<BigUint>;
    
    #[only_owner]
    #[endpoint(setMinStakeLimit)]
    fn set_min_stake_limit(&self, min_stake_limit: BigUint) {
        self.min_stake_limit().set(min_stake_limit);
    }

    //
    #[view(getMaxStakeLimit)]
    #[storage_mapper("max_stake_limit")]
    fn max_stake_limit(&self) -> SingleValueMapper<BigUint>;
    
    #[only_owner]
    #[endpoint(setMaxStakeLimit)]
    fn set_max_stake_limit(&self, max_stake_limit: BigUint) {
        self.max_stake_limit().set(max_stake_limit);
    }

    //
    #[view(getStakeTokenPrice)]
    #[storage_mapper("stake_token_price")]
    fn stake_token_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getRewardTokenPrice)]
    #[storage_mapper("reward_token_price")]
    fn reward_token_price(&self) -> SingleValueMapper<BigUint>;

    #[only_owner]
    #[endpoint(setStakeTokenToRewardTokenRate)]
    fn set_stake_token_to_reward_token_rate(&self, stake_token_price: BigUint, reward_token_price: BigUint) {
        self.stake_token_price().set(stake_token_price);
        self.reward_token_price().set(reward_token_price);
    }

    //
    #[view(getLockPeriod)]
    #[storage_mapper("lock_period")]
    fn lock_period(&self) -> SingleValueMapper<u64>;

    #[only_owner]
    #[endpoint(setLockPeriod)]
    fn set_lock_period(&self, lock_period: u64) {
        self.lock_period().set(lock_period);
    }

    //
    #[view(getUndelegationPeriod)]
    #[storage_mapper("undelegation_period")]
    fn undelegation_period(&self) -> SingleValueMapper<u64>;

    #[only_owner]
    #[endpoint(setUndelegationPeriod)]
    fn set_undelegation_period(&self, undelegation_period: u64) {
        self.undelegation_period().set(undelegation_period);
    }

    //
    #[view(getClaimLockPeriod)]
    #[storage_mapper("claim_lock_period")]
    fn claim_lock_period(&self) -> SingleValueMapper<u64>;

    #[only_owner]
    #[endpoint(setClaimLockPeriod)]
    fn set_claim_lock_period(&self, claim_lock_period: u64) {
        self.claim_lock_period().set(claim_lock_period);
    }
    
    ///

    #[view(getRewardAprs)]
    #[storage_mapper("reward_aprs")]
    fn reward_aprs(&self) -> VecMapper<RewardApr>;

    #[only_owner]
    #[endpoint(addRewardApr)]
    fn add_reward_apr(
        &self,
        apr: u32,
        #[var_args] opt_start_timestamp: OptionalValue<u64>
    ) {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let new_id = self.reward_aprs().len() + 1;
        let start_timestamp = match opt_start_timestamp {
            OptionalValue::Some(v) => v,
            OptionalValue::None => self.blockchain().get_block_timestamp(),
        };

        // if there is already reward_apr, update end_timestamp of previous reward_apr
        if new_id > 1 {
            let mut previous_reward_apr = self.reward_aprs().get(new_id - 1);
            previous_reward_apr.end_timestamp = current_timestamp;
            self.reward_aprs().set(new_id - 1, &previous_reward_apr);
        }

        self.reward_aprs().push(
            &RewardApr {
                id: new_id,
                apr: apr,
                start_timestamp: start_timestamp,
                end_timestamp: 0,
            }
        );
    }

    #[only_owner]
    #[endpoint(clearRewardAprs)]
    fn clear_reward_aprs(&self) {
        self.reward_aprs().clear();
    }

    ///

    #[view(getPaused)]
    #[storage_mapper("paused")]
    fn paused(&self) -> SingleValueMapper<bool>;

    #[only_owner]
    #[endpoint(pause)]
    fn pause(&self) {
        self.paused().set(true);
    }

    #[only_owner]
    #[endpoint(unpause)]
    fn unpause(&self) {
        self.paused().set(false);
    }

    /// 
    
    #[view(getTotalStakedAmount)]
    #[storage_mapper("total_staked_amount")]
    fn total_staked_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getStakerAddresses)]
    #[storage_mapper("staker_addresses")]
    fn staker_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getStakeAmount)]
    #[storage_mapper("staked_amount")]
    fn staked_amount(&self, staker_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getLockEndTimestamp)]
    #[storage_mapper("lock_end_timestamp")]
    fn lock_end_timestamp(&self, staker_address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getUnstakedAmount)]
    #[storage_mapper("unstaked_amount")]
    fn unstaked_amount(&self, staker_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getUndelegationEndTimestamp)]
    #[storage_mapper("undelegation_end_timestamp")]
    fn undelegation_end_timestamp(&self, staker_address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getCollectableAmount)]
    #[storage_mapper("collectable_amount")]
    fn collectable_amount(&self, staker_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getRewardAmount)]
    #[storage_mapper("reward_amount")]
    fn reward_amount(&self, staker_address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getLastRewardUpdatedTimestamp)]
    #[storage_mapper("last_reward_updated_timestamp")]
    fn last_reward_updated_timestamp(&self, staker_address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getLastClaimTimestamp)]
    #[storage_mapper("last_claim_timestamp")]
    fn last_claim_timestamp(&self, staker_address: &ManagedAddress) -> SingleValueMapper<u64>;
}