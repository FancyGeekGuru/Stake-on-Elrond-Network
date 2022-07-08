#![no_std]
#![feature(generic_associated_types)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod state;
mod storage;
mod event;
mod logic;

use crate::state::{ StakeSetting, StakeAccount };

#[elrond_wasm::derive::contract]
pub trait OdinStakeV2:
    logic::LogicModule
    + storage::StorageModule
    + event::EventModule
{
    #[init]
    fn init(&self,
        stake_token_id: TokenIdentifier,
        reward_token_id: TokenIdentifier,
        min_stake_limit: BigUint,
        max_stake_limit: BigUint,
        lock_period: u64,
        undelegation_period: u64,
        claim_lock_period: u64,
        stake_token_price: BigUint,
        reward_token_price: BigUint,
    ){
        self.stake_token_id().set(stake_token_id);
        self.reward_token_id().set(reward_token_id);
        self.min_stake_limit().set(min_stake_limit);
        self.max_stake_limit().set(max_stake_limit);
        self.lock_period().set(lock_period);
        self.undelegation_period().set(undelegation_period);
        self.claim_lock_period().set(claim_lock_period);
        self.stake_token_price().set(stake_token_price);
        self.reward_token_price().set(reward_token_price);

        self.paused().set(false);    // live
    }

    

    ///
    #[view(viewStakeAccount)]
    fn view_stake_account(
        &self,
        caller: &ManagedAddress,
    ) -> StakeAccount<Self::Api> {
        let reward_amount = self.get_current_reward(caller);

        let current_timestamp = self.blockchain().get_block_timestamp();
        let mut collectable_amount = self.collectable_amount(caller).get();
        let mut unstaked_amount = self.unstaked_amount(caller).get();
        if self.undelegation_end_timestamp(caller).get() <= current_timestamp && self.unstaked_amount(caller).get() > BigUint::zero() {
            collectable_amount += &self.unstaked_amount(caller).get();
            unstaked_amount = BigUint::zero();
        }
        
        StakeAccount {
            address: caller.clone(),
            staked_amount: self.staked_amount(caller).get(),
            lock_end_timestamp: self.lock_end_timestamp(caller).get(),
            unstaked_amount: unstaked_amount,
            undelegation_end_timestamp: self.undelegation_end_timestamp(caller).get(),
            collectable_amount: collectable_amount,
            reward_amount: reward_amount,
            last_reward_updated_timestamp: self.last_reward_updated_timestamp(caller).get(),
            last_claim_timestamp: self.last_claim_timestamp(caller).get(),
        }
    }

    #[view(viewStakeAccounts)]
    fn view_stake_accounts(&self) -> MultiValueEncoded<StakeAccount<Self::Api>> {
        let mut items_vec = MultiValueEncoded::new();

        for address in self.staker_addresses().iter() {
            let stake_account = self.view_stake_account(&address);
            items_vec.push(stake_account);
        }

        items_vec
    }

    #[view(getCurrentApr)]
    fn get_current_apr(&self) -> u32 {
        let count = self.reward_aprs().len();
        if count > 0 {
            return self.reward_aprs().get(count).apr;
        } else {
            return 0;
        }
    }

    #[view(viewStakeSetting)]
    fn view_stake_setting(&self) -> StakeSetting<Self::Api> {
        // count number of stakers who have staked tokens
        let mut number_of_stakers = 0u32;
        for address in self.staker_addresses().iter() {
            if self.staked_amount(&address).get() != BigUint::zero() {
                number_of_stakers += 1;
            }
        }

        StakeSetting {
            stake_token_id: self.stake_token_id().get(),
            reward_token_id: self.reward_token_id().get(),
            min_stake_limit: self.min_stake_limit().get(),
            max_stake_limit: self.max_stake_limit().get(), // 0 for no max limit
            lock_period: self.lock_period().get(),
            undelegation_period: self.undelegation_period().get(),
            claim_lock_period: self.claim_lock_period().get(),
            stake_token_price: self.stake_token_price().get(),
            reward_token_price: self.reward_token_price().get(),
            apr: self.get_current_apr(),
            total_staked_amount: self.total_staked_amount().get(),
            number_of_stakers: number_of_stakers,
        }
    }

    ///
    #[only_owner]
    #[endpoint(withdrawFunds)]
    fn withdraw_funds(&self,
        #[var_args] opt_token_id: OptionalValue<TokenIdentifier>,
        #[var_args] opt_token_nonce: OptionalValue<u64>,
        #[var_args] opt_token_amount: OptionalValue<BigUint>
    ) {
        // if token_id is not given, set it to eGLD
        let token_id = match opt_token_id {
            OptionalValue::Some(v) => v,
            OptionalValue::None => TokenIdentifier::egld()
        };

        // if token_id is not given, set it to 0
        let token_nonce = match opt_token_nonce {
            OptionalValue::Some(v) => v,
            OptionalValue::None => 0,
        };

        // if token_amount is not given, set it to balance of SC - max value to withdraw
        let token_amount = match opt_token_amount {
            OptionalValue::Some(v) => v,
            OptionalValue::None => self.blockchain().get_sc_balance(&token_id, 0)
        };

        self.send().direct(&self.blockchain().get_caller(), &token_id, token_nonce, &token_amount, &[]);
    }
}