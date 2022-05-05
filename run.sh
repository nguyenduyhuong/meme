 contractId="test7.duyhuongtest.testnet"
 #near call $contractId --accountId=duyhuongtest.testnet ft_transfer '{"receiver_id": "devhuong.testnet", "amount": "2000000000"}' --amount=0.000000000000000000000001

 # near call $contractId --accountId=duyhuongtest.testnet storage_deposit '{"account_id": "devhuong.testnet"}' --amount=0.00125
 #near call $contractId --accountId=duyhuongtest.testnet ft_transfer '{"receiver_id": "devhuong.testnet", "amount": "9999898096800"}' --amount=0.000000000000000000000001

# near view $contractId ft_balance_of '{"account_id": "ref-finance-101.testnet"}'
# near view $contractId ft_balance_of '{"account_id": "duyhuongtest.testnet"}'

 #near view wrap.testnet ft_balance_of '{"account_id": "test3.duyhuongtest.testnet"}'
# near view $contractId ft_balance_of '{"account_id": "test.duyhuongtest.testnet"}'

 near call $contractId --accountId=devhuong.testnet ft_transfer '{"receiver_id": "ref-finance-101.testnet", "amount": "99998980968"}' --amount=0.000000000000000000000001 --gas=300000000000000
near view $contractId ft_balance_of '{"account_id": "devhuong.testnet"}'
near view $contractId ft_balance_of '{"account_id": "huong1728.testnet"}'




