use near_sdk::{env,ext_contract,AccountId,Gas,Balance};
use near_sdk::json_types::{U128};

const GAS: Gas = Gas(5_000_000_000_000);

#[ext_contract(ext_wnear)]
trait wnear {
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;
    fn ft_balance_of(&self, account_id: String) -> U128;
}
#[ext_contract(ext_ft)]
trait FungibleToken {
    fn ft_balance_of(&self, account_id: String) -> U128;
}
#[ext_contract(ext_ref)]
pub trait RefFinance {
     fn add_liquidity(
        &mut self,
        pool_id: u64,
        amounts: Vec<U128>,
        min_amounts: Option<Vec<U128>>,
    ) -> U128;
}
#[ext_contract(ext_self)]
pub trait ContractA {
    fn my_callback(&self) -> String;
}
pub struct AddLiquidity {
    poolId : u32,
    token : AccountId,
    minimumtoAddLiquidity : u128,
}

impl AddLiquidity {
    pub fn new(pooldId: u32,token :AccountId,minimumtoAddLiquidity : u128) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        AddLiquidity {
           poolId : pooldId,
           token : token.into(),
           minimumtoAddLiquidity : minimumtoAddLiquidity
        }
    }
     fn internal_swap( &mut self, amount : Balance)  {
        let contractRef: AccountId = "ref-finance-101.testnet".parse().unwrap();
        let s :String =  format!("{{\"force\":0,\"actions\":[{{\"pool_id\":{},\"token_in\":\"wrap.testnet\",\"token_out\":\"{}\",\"amount_in\":\"{}\",\"min_amount_out\":\"0\"}}]}}",self.poolId,self.token,amount);
      //  let s :String =  format!("{{\"force\":0,\"actions\":[{{\"pool_id\":387,\"token_in\":\"wrap.testnet\",\"token_out\":\"token11.duyhuongtest.testnet\",\"amount_in\":\"10000000000000000000000\",\"min_amount_out\":\"82323310\"}}]}}");

        ext_wnear::ft_transfer_call(contractRef,U128(amount),None, s,"wrap.testnet".parse().unwrap(), 3000000000000000000000000,GAS);
    }

     fn internal_add_liquidity( &mut self,amounts: Vec<U128>) {
        ext_ref::add_liquidity(self.poolId.into(),amounts,None,"ref-finance-101.testnet".parse().unwrap(),3000000000000000000000000,GAS);
    }

    pub fn swapAndLiquidify(&mut self,amount : Balance) {
        let this_contract   = env::current_account_id();
        let amount_wnear = ext_ft::ft_balance_of(
            this_contract,                                  // method arguments (ft_balance_of takes an account id)
           "wrap.testnet".parse().unwrap(),               // the recipient of this ActionReceipt (contract account id)
            0,                                                          // amount of yoctoNEAR to attach
            GAS,                                               // gas to attach
        );
        let amount_token =   ext_ft::ft_balance_of(
            this_contract,                                  // method arguments (ft_balance_of takes an account id)
            self.token,               // the recipient of this ActionReceipt (contract account id)
            0,                                                          // amount of yoctoNEAR to attach
            GAS,                                               // gas to attach
        );
        if(amount_token > self.minimumtoAddLiquidity){
            let half : u128 = amount /2; 
            let left : u128 = amount-half;
            self.internal_swap(half);
            self.internal_add_liquidity([]);
        }
      
    }
}

