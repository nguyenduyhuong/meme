#near call wrap.testnet --accountId=duyhuongtest.testnet ft_transfer '{"receiver_id": "ref-finance-101.testnet", "amount": "1000000000000000000000"}' --amount=0.000000000000000000000001
 #near call ref-finance-101.testnet --accountId=duyhuongtest.testnet register_tokens '{"token_ids": ["skyward.fakes.testnet"]}' --amount=0.000000000000000000000001

#near call wrap.testnet --accountId=duyhuongtest.testnet ft_transfer_call '{"receiver_id":"ref-finance-101.testnet","amount":"1000000000000000000000","msg":"{"force":0,"actions":[{"pool_id":387,"token_in":"wrap.testnet","token_out":"token11.duyhuongtest.testnet","amount_in":"1000000000000000000000","min_amount_out":"82323310"}]}"}' --amount=0.000000000000000000000001
#near call ref-finance-101.testnet swap '{"actions":[{ "pool_id" : 50, "token_in" : "skyward.fakes.testnet","amount_in" : "1000000000000000", "token_out" :"wrap.testnet","min_amount_out":"8247325"}]}' --amount=0.00125 --accountId=duyhuongtest.testnet


near call wrap.testnet --accountId=devhuong.testnet ft_transfer_call '{
   "receiver_id": "ref-finance-101.testnet",
  "amount": "1000000000000000000000",
  "msg": "{\"force\":0,\"actions\":[{\"pool_id\":411,\"token_in\":\"wrap.testnet\",\"token_out\":\"test.duyhuongtest.testnet\",\"amount_in\":\"1000000000000000000000\",\"min_amount_out\":\"0\"}]}"
}' --amount=0.000000000000000000000001 --gas=300000000000000

