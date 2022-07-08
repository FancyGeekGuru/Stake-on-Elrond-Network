elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use core::cmp::{min, max};

const DAY_IN_SECONDS: u64 = 3600 * 24;
// const DAY_IN_SECONDS: u64 = 10;
const YEAR_IN_DAYS: u64 = 365;
const TOTAL_PERCENTAGE: u64 = 10000;

#[elrond_wasm::module]
pub trait LogicModule:
    crate::storage::StorageModule
    + crate::event::EventModule
{
    #[payable("*")]
    #[endpoint(stake)]
    fn stake(
        &self,
        #[payment_token] stake_token_id: TokenIdentifier,
        #[payment_amount] stake_amount: BigUint
    ) {
        self.require_activation();

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        require!(
            stake_token_id == self.stake_token_id().get(),
            "invalid stake_token_id"
        );

        self.check_existance_of_stake_account(&caller);
        self.update_stake_account();

        require!(
            stake_amount.clone() + &self.staked_amount(&caller).get() >= self.min_stake_limit().get(),
            "cannot stake less than min_stake_limit in total"
        );

        // if max_stake_limit is zero, it means no max limit
        if self.max_stake_limit().get() != BigUint::zero() {
            require!(
                stake_amount.clone() + &self.staked_amount(&caller).get()  <= self.max_stake_limit().get(),
                "cannot stake more than max_stake_limit in total"
            );
        }

        self.staked_amount(&caller).update(|v| *v += &stake_amount);
        self.lock_end_timestamp(&caller).set(current_timestamp + self.lock_period().get());
        // for lock claim
        self.last_claim_timestamp(&caller).set(current_timestamp);

        self.total_staked_amount().update(|v| *v += &stake_amount);

        self.stake_event(
            caller,
            stake_token_id,
            stake_amount,
            current_timestamp
        );
    }

    #[endpoint(unstake)]
    fn unstake(
        &self,
        #[var_args] opt_unstake_amount: OptionalValue<BigUint>
    ) {
        self.require_activation();

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        self.update_stake_account();

        require!(
            self.staked_amount(&caller).get() != BigUint::zero(),
            "nothing to unstake"
        );
        require!(
            current_timestamp >= self.lock_end_timestamp(&caller).get(),
            "cannot unstake before lock period"
        );

        // if unstake_amount is not given, unstake all staked balance
        let unstake_amount = match opt_unstake_amount {
            OptionalValue::Some(value) => min(value, self.staked_amount(&caller).get()),
            OptionalValue::None => self.staked_amount(&caller).get()
        };

        self.staked_amount(&caller).update(|v| *v -= &unstake_amount);
        self.unstaked_amount(&caller).update(|v| *v += &unstake_amount);
        self.undelegation_end_timestamp(&caller).set(current_timestamp + self.undelegation_period().get());

        self.unstake_event(
            caller,
            self.stake_token_id().get(),
            unstake_amount,
            current_timestamp,
        );
    }

    #[endpoint(claim)]
    fn claim(&self) {
        self.require_activation();

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        self.update_stake_account();

        require!(
            self.last_claim_timestamp(&caller).get() + self.claim_lock_period().get() <= current_timestamp,
            "you cannot claim before claim_lock_period"
        );

        let reward_token_id = self.reward_token_id().get();
        let reward_amount = self.get_current_reward(&caller);
        let stake_token_id = self.stake_token_id().get();
        let collectable_amount = self.collectable_amount(&caller).get();

        require!(
            reward_amount != BigUint::zero() || collectable_amount != BigUint::zero(),
            "no rewards or collectable tokens to be claimed"
        );

        if reward_amount != BigUint::zero() {
            self.reward_amount(&caller).set(BigUint::zero());
            self.last_reward_updated_timestamp(&caller).set(current_timestamp);
            self.last_claim_timestamp(&caller).set(current_timestamp);

            require!(
                self.blockchain().get_sc_balance(&reward_token_id, 0) >= reward_amount,
                "not enough rewarding tokens in smart contract"
            );
            
            self.send().direct(&caller, &reward_token_id, 0, &reward_amount, b"reward");

            self.claim_event(
                caller.clone(),
                reward_token_id,
                reward_amount,
                current_timestamp,
            );
        }

        if collectable_amount != BigUint::zero() {
            self.collectable_amount(&caller).set(BigUint::zero());
            self.total_staked_amount().update(|v| *v -= & collectable_amount);

            require!(
                self.blockchain().get_sc_balance(&stake_token_id, 0) >= collectable_amount,
                "not enough staking tokens in smart contract"
            );
            
            self.send().direct(&caller, &stake_token_id, 0, &collectable_amount, b"collect");

            self.collect_event(
                caller,
                stake_token_id,
                collectable_amount,
                current_timestamp,
            );
        }
    }

    #[endpoint]
    fn restake(&self) {
        self.require_activation();

        let stake_token_id = self.stake_token_id().get();
        let reward_token_id = self.reward_token_id().get();
        require!(
            stake_token_id == reward_token_id,
            "stake_token_id and reward_token_id should be the same for restake"
        );

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        self.update_stake_account();

        require!(
            self.last_claim_timestamp(&caller).get() + self.claim_lock_period().get() <= current_timestamp,
            "you cannot restake before claim_lock_period"
        );
        
        let reward_amount = self.get_current_reward(&caller);       
        require!(
            reward_amount != BigUint::zero(),
            "no rewards for restake"
        );

        // claim
        self.reward_amount(&caller).set(BigUint::zero());
        self.last_reward_updated_timestamp(&caller).set(current_timestamp);
        self.last_claim_timestamp(&caller).set(current_timestamp);

        self.claim_event(
            caller.clone(),
            reward_token_id,
            reward_amount.clone(),
            current_timestamp,
        );

        // stake
        self.staked_amount(&caller).update(|v| *v += &reward_amount);
        // self.lock_end_timestamp(&caller).set(current_timestamp + self.lock_period().get());

        self.total_staked_amount().update(|v| *v += &reward_amount);

        self.stake_event(
            caller.clone(),
            stake_token_id,
            reward_amount,
            current_timestamp
        );
    }

    ///
    #[inline]
    fn update_stake_account(&self) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let new_reward_amount = self.get_current_reward(&caller);

        self.reward_amount(&caller).set(new_reward_amount);
        self.last_reward_updated_timestamp(&caller).set(current_timestamp);

        if self.undelegation_end_timestamp(&caller).get() <= current_timestamp && self.unstaked_amount(&caller).get() > BigUint::zero() {
            self.collectable_amount(&caller).update(|v| *v += &self.unstaked_amount(&caller).get());
            self.unstaked_amount(&caller).set(BigUint::zero());
        }
    }
    
    #[view(getCurrentReward)]
    fn get_current_reward(
        &self,
        caller: &ManagedAddress
    ) -> BigUint {
        let current_timestamp = self.blockchain().get_block_timestamp();

        let increased_reward = if self.staked_amount(caller).get() != BigUint::zero() {
            let mut sum = BigUint::zero();
            
            for reward_apr in self.reward_aprs().iter() {
                let duration = if reward_apr.end_timestamp == 0 {   // if last reward_apr
                    current_timestamp - max(reward_apr.start_timestamp, self.last_reward_updated_timestamp(caller).get())
                } else if self.last_reward_updated_timestamp(caller).get() <= reward_apr.start_timestamp {
                    reward_apr.end_timestamp - reward_apr.start_timestamp
                } else if self.last_reward_updated_timestamp(caller).get() < reward_apr.end_timestamp {
                    reward_apr.end_timestamp - self.last_reward_updated_timestamp(caller).get()
                } else {
                    0
                };

                if duration > 0 {
                    sum += self.staked_amount(caller).get() * reward_apr.apr * duration * &self.reward_token_price().get() / TOTAL_PERCENTAGE / (DAY_IN_SECONDS * YEAR_IN_DAYS) / &self.stake_token_price().get();
                }
            }

            sum
        } else {
            BigUint::zero()
        };

        increased_reward + &self.reward_amount(caller).get()
    }

    #[inline]
    fn require_activation(&self) {
        require!(
            self.paused().get() == false,
            "staking is not live"
        );

        require!(
            self.reward_aprs().len() != 0,
            "no reward_apr is set"
        );
    }

    /*
        check if stake_account of caller exists
        if not exist, create a new one with zero values
    */
    #[inline]
    fn check_existance_of_stake_account(
        &self,
        caller: &ManagedAddress,
    ) {
        if !self.staker_addresses().contains(caller) {
            self.staker_addresses().insert(caller.clone());
        }
    }
}