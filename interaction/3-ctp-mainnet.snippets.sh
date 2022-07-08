PROXY=https://gateway.elrond.com
CHAIN_ID="1"
WALLET="./wallets/odin.pem"
ADDRESS=$(erdpy data load --key=address-ctp-mainnet)

###############################################################################

STAKE_TOKEN_ID="CTP-298075"
STAKE_TOKEN_ID_HEX="0x$(echo -n ${STAKE_TOKEN_ID} | xxd -p -u | tr -d '\n')"
STAKE_TOKEN_ID_ONLY_HEX="$(echo -n ${STAKE_TOKEN_ID} | xxd -p -u | tr -d '\n')"

REWARD_TOKEN_ID="CTP-298075"
REWARD_TOKEN_ID_HEX="0x$(echo -n ${REWARD_TOKEN_ID} | xxd -p -u | tr -d '\n')"

MIN_STAKE_LIMIT=0
MAX_STAKE_LIMIT=0 # no max limit

LOCKING_PERIOD=2592000 # 30 days
UNDELEGATION_PERIOD=0
CLAIM_LOCK_PERIOD=604800 # 7 days

APR=20000 # 200%

STAKE_TOKEN_PRICE=1
REWARD_TOKEN_PRICE=1

STAKE="stake"
STAKE_ONLY_HEX="$(echo -n ${STAKE} | xxd -p -u | tr -d '\n')"


deploy() {
    erdpy --verbose contract deploy \
    --project=${PROJECT} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=60000000 \
    --arguments ${STAKE_TOKEN_ID_HEX} ${REWARD_TOKEN_ID_HEX} ${MIN_STAKE_LIMIT} ${MAX_STAKE_LIMIT} ${LOCKING_PERIOD} ${UNDELEGATION_PERIOD} ${CLAIM_LOCK_PERIOD} ${STAKE_TOKEN_PRICE} ${REWARD_TOKEN_PRICE} \
    --send \
    --metadata-payable \
    --outfile="deploy-ctp-mainnet.interaction.json" \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} || return

    ADDRESS=$(erdpy data parse --file="deploy-ctp-mainnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-ctp-mainnet --value=${ADDRESS}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --metadata-payable \
    --gas-limit=50000000 \
    --arguments ${STAKE_TOKEN_ID_HEX} ${REWARD_TOKEN_ID_HEX} ${MIN_STAKE_LIMIT} ${MAX_STAKE_LIMIT} ${LOCKING_PERIOD} ${UNDELEGATION_PERIOD} ${CLAIM_LOCK_PERIOD} ${STAKE_TOKEN_PRICE} ${REWARD_TOKEN_PRICE} ${UNSTAKE_BEFORE_LOCK_PERIOD_PENALTY}
}

addRewardApr() {
    erdpy --verbose contract call ${ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="addRewardApr" \
    --arguments ${APR}
}

setLockPeriod() {
    erdpy --verbose contract call ${ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setLockPeriod" \
    --arguments ${LOCKING_PERIOD}
}

stake2000() {
    erdpy --verbose tx new --receiver ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=10000000 \
    --data="ESDTTransfer@${STAKE_TOKEN_ID_ONLY_HEX}@6c6b935b8bbd400000@${STAKE_ONLY_HEX}" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

stake600() {
    erdpy --verbose tx new --receiver ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=10000000 \
    --data="ESDTTransfer@${STAKE_TOKEN_ID_ONLY_HEX}@2086ac351052600000@${STAKE_ONLY_HEX}" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

stake5000() {
    erdpy --verbose tx new --receiver ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=10000000 \
    --data="ESDTTransfer@${STAKE_TOKEN_ID_ONLY_HEX}@010f0cf064dd59200000@${STAKE_ONLY_HEX}" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

unstake() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="unstake" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

claim() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="claim" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

restake() {
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="restake" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

withdrawFunds() {
    erdpy --verbose contract call ${ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="withdrawFunds" \
    --arguments ${STAKE_TOKEN_ID_HEX}
}

setMinStakeLimit() {
    erdpy --verbose contract call ${ADDRESS} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setMinStakeLimit" \
    --arguments ${MIN_STAKE_LIMIT}
}

# config

getRewardAprs() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getRewardAprs"
}

getStakerAddresses() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getStakerAddresses" 
}

viewStakeAccounts() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="viewStakeAccounts" 
}

getTotalSupply() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} \
    --function="getTotalSupply"
}
