contractId="addliquid.duyhuongtest.testnet"

near create-account $contractId --masterAccount duyhuongtest.testnet --initialBalance 3
near deploy --wasmFile res/addliquid.wasm --accountId $contractId


 near call $contractId new '{"pool_id": 380, "token":"token2.duyhuongtest.testnet","owner_id" : "'$contractId'","ref_contract" : "ref-finance-101.testnet","wrap_id":"wrap.testnet"}' --accountId duyhuongtest.testnet
####### setup
# near call token2.duyhuongtest.testnet --accountId=duyhuongtest.testnet storage_deposit '{"account_id": "'$contractId'"}' --amount=0.00125
# near call token2.duyhuongtest.testnet --accountId=duyhuongtest.testnet ft_transfer '{"receiver_id": "'$contractId'", "amount": "10000000000000"}' --amount=0.000000000000000000000001
# near call wrap.testnet --accountId=duyhuongtest.testnet storage_deposit '{"account_id": "'$contractId'"}' --amount=0.00125
#  near call wrap.testnet --accountId=duyhuongtest.testnet ft_transfer '{"receiver_id": "'$contractId'", "amount": "1000000000000000000000"}' --amount=0.000000000000000000000001
# near call ref-finance-101.testnet storage_deposit '' --accountId $contractId --amount 0.1
# near call ref-finance-101.testnet register_tokens '{"token_ids": ["token2.duyhuongtest.testnet", "wrap.testnet"]}' --accountId $contractId --amount=0.000000000000000000000001


# near view token2.duyhuongtest.testnet ft_balance_of '{"account_id": "'$contractId'"}'
# near view token2.duyhuongtest.testnet ft_balance_of '{"account_id": "ref-finance-101.testnet"}'
# near call $contractId swapAndLiquidify '{"amountSwap":1000,"amounts": ["999999982000", "49206020910007191848"]}' --accountId $contractId --amount=0.000000000000000000000001 --gas=300000000000000
# near view token2.duyhuongtest.testnet ft_balance_of '{"account_id": "ref-finance-101.testnet"}'


# near call wrap.testnet --accountId=duyhuongtest.testnet storage_deposit '{"account_id": "'$contractId'"}' --amount=0.00225
# near call wrap.testnet --accountId=duyhuongtest.testnet ft_transfer '{"receiver_id": "'$contractId'", "amount": "100000000000000000"}' --amount=0.000000000000000000000001

4734086



 



