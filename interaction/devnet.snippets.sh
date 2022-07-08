PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WALLET="./wallets/shard1-wallet.pem"
ADDRESS=$(erdpy data load --key=address-devnet)

###############################################################################

CALLER_ADDRESS="erd1ygdttzrulwfspme2s4qrx5y2qyfqalju0k2vcyy8z3979whlj9qssl5uay"
CALLER_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${CALLER_ADDRESS})"
CALLER_ADDRESS_ONLY_HEX="$(erdpy wallet bech32 --decode ${CALLER_ADDRESS})"

STAKE_TOKEN_ID="MEX-450e50"
STAKE_TOKEN_ID_HEX="0x$(echo -n ${STAKE_TOKEN_ID} | xxd -p -u | tr -d '\n')"
STAKE_TOKEN_ID_ONLY_HEX="$(echo -n ${STAKE_TOKEN_ID} | xxd -p -u | tr -d '\n')"

REWARD_TOKEN_ID="MEX-450e50"
REWARD_TOKEN_ID_HEX="0x$(echo -n ${REWARD_TOKEN_ID} | xxd -p -u | tr -d '\n')"

MIN_STAKE_LIMIT=0
MAX_STAKE_LIMIT=0 # no max limit

# LOCKING_PERIOD=432000
# UNDELEGATION_PERIOD=432000
# CLAIM_LOCK_PERIOD=300

LOCKING_PERIOD=0
UNDELEGATION_PERIOD=0
CLAIM_LOCK_PERIOD=0

APR=10000 # 100%

STAKE_TOKEN_PRICE=1
REWARD_TOKEN_PRICE=1

STAKE="stake"
STAKE_ONLY_HEX="$(echo -n ${STAKE} | xxd -p -u | tr -d '\n')"


deploy() {
    erdpy --verbose contract deploy \
    --project=${PROJECT} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=100000000 \
    --arguments ${STAKE_TOKEN_ID_HEX} ${REWARD_TOKEN_ID_HEX} ${MIN_STAKE_LIMIT} ${MAX_STAKE_LIMIT} ${LOCKING_PERIOD} ${UNDELEGATION_PERIOD} ${CLAIM_LOCK_PERIOD} ${STAKE_TOKEN_PRICE} ${REWARD_TOKEN_PRICE} \
    --send \
    --metadata-payable \
    --outfile="deploy-devnet.interaction.json" \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} || return

    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}

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
