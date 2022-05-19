contractId="addliquid.duyhuongtest.testnet"


near call $contractId set_owner '{"owner_id" :"huong"}' --accountId $contractId --amount=0.000000000000000000000001


near view ref-finance-101.testnet get_deposits  '{"account_id": "'$contractId'"}'
 near view ref-finance-101.testnet get_return '{"pool_id":380,"token_in":"token2.duyhuongtest.testnet","amount_in":"4734086","token_out":"wrap.testnet"}'
 near view ref-finance-101.testnet get_return '{"pool_id":380,"token_in":"wrap.testnet","amount_in":"49206020910007191847","token_out":"token2.duyhuongtest.testnet"}'



