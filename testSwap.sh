 contractId="test4.duyhuongtest.testnet"
 
 

near call wrap.testnet --accountId=duyhuongtest.testnet storage_deposit '{"account_id": "test6.duyhuongtest.testnet"}' --amount=0.00125
 near call wrap.testnet --accountId=duyhuongtest.testnet ft_transfer '{"receiver_id": "test6.duyhuongtest.testnet", "amount": "20000000000000000000000"}' --amount=0.000000000000000000000001
 near call wrap.testnet --accountId=duyhuongtest.testnet ft_balance_of '{"account_id": "test6.duyhuongtest.testnet"}' --amount=0.00125


