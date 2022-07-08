////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    odin_stake_v2
    (
        addRewardApr
        claim
        clearRewardAprs
        getClaimLockPeriod
        getCollectableAmount
        getCurrentApr
        getCurrentReward
        getLastClaimTimestamp
        getLastRewardUpdatedTimestamp
        getLockEndTimestamp
        getLockPeriod
        getMaxStakeLimit
        getMinStakeLimit
        getPaused
        getRewardAmount
        getRewardAprs
        getRewardTokenId
        getRewardTokenPrice
        getStakeAmount
        getStakeTokenId
        getStakeTokenPrice
        getStakerAddresses
        getTotalStakedAmount
        getUndelegationEndTimestamp
        getUndelegationPeriod
        getUnstakedAmount
        pause
        restake
        setClaimLockPeriod
        setLockPeriod
        setMaxStakeLimit
        setMinStakeLimit
        setRewardTokenId
        setStakeTokenId
        setStakeTokenToRewardTokenRate
        setUndelegationPeriod
        stake
        unpause
        unstake
        viewStakeAccount
        viewStakeAccounts
        viewStakeSetting
        withdrawFunds
    )
}

elrond_wasm_node::wasm_empty_callback! {}
