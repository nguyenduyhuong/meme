#!/bin/bash
# set -e
# cd "`dirname $0`"
# cargo build --all --target wasm32-unknown-unknown --release
# cp target/wasm32-unknown-unknown/release/*.wasm ./res/

 contractId="test7.duyhuongtest.testnet"

near create-account $contractId --masterAccount duyhuongtest.testnet --initialBalance 3
near deploy --wasmFile res/fungible_token.wasm --accountId $contractId

 near call $contractId new '{"owner_id": "duyhuongtest.testnet", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Meme Coin", "symbol": "testNear5","decimals": 8 }}' --accountId token3.duyhuongtest.testnet

near call $contractId --accountId=duyhuongtest.testnet storage_deposit '{"account_id": "huong1728.testnet"}' --amount=0.00125
near call $contractId --accountId=duyhuongtest.testnet storage_deposit '{"account_id": "ref-finance-101.testnet"}' --amount=0.00125

#   near call $contractId --accountId=duyhuongtest.testnet ft_transfer '{"receiver_id": "ref-finance-101.testnet", "amount": "200000000"}' --amount=0.000000000000000000000001
near view $contractId ft_balance_of '{"account_id": "duyhuongtest.testnet"}'
near view $contractId ft_balance_of '{"account_id": "ref-finance-101.testnet"}'
near view $contractId ft_balance_of '{"account_id": "huong1728.testnet"}'


near call $contractId --accountId=duyhuongtest.testnet storage_deposit '{"account_id": "devhuong.testnet"}' --amount=0.00125
 near call $contractId --accountId=duyhuongtest.testnet ft_transfer '{"receiver_id": "devhuong.testnet", "amount": "9999898096800"}' --amount=0.000000000000000000000001

 near call wrap.testnet --accountId=duyhuongtest.testnet storage_deposit '{"account_id": "test7.duyhuongtest.testnet"}' --amount=0.00125
 near call wrap.testnet --accountId=duyhuongtest.testnet ft_transfer '{"receiver_id": "test7.duyhuongtest.testnet", "amount": "20000000000000000000000"}' --amount=0.000000000000000000000001
 near call wrap.testnet --accountId=duyhuongtest.testnet ft_balance_of '{"account_id": "test7.duyhuongtest.testnet"}' --amount=0.00125






 

