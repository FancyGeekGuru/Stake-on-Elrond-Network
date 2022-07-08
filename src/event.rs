elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait EventModule {
    #[event("stake")]
    fn stake_event(
        &self,
        #[indexed] user_address: ManagedAddress,
        #[indexed] stake_token: TokenIdentifier,
        #[indexed] stake_amount: BigUint,
        #[indexed] stake_timestamp: u64,
    );

    #[event("unstake")]
    fn unstake_event(
        &self,
        #[indexed] user_address: ManagedAddress,
        #[indexed] unstake_token: TokenIdentifier,
        #[indexed] unstake_amount: BigUint,
        #[indexed] unstake_timestamp: u64,
    );

    #[event("claim")]
    fn claim_event(
        &self,
        #[indexed] user_address: ManagedAddress,
        #[indexed] claim_token: TokenIdentifier,
        #[indexed] claim_amount: BigUint,
        #[indexed] claim_timestamp: u64,
    );

    #[event("collect")]
    fn collect_event(
        &self,
        #[indexed] user_address: ManagedAddress,
        #[indexed] collect_token: TokenIdentifier,
        #[indexed] collect_amount: BigUint,
        #[indexed] collect_timestamp: u64,
    );
}