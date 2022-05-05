
 use near_sdk::{env,Promise};
use near_sdk::{ext_contract,AccountId,Gas};
use near_sdk::json_types::{ValidAccountId,U128};

const GAS: Gas = Gas(5_000_000_000_000);

#[ext_contract(ext_ref)]
pub trait RefFinance {
    pub fn add_liquidity(
        &mut self,
        pool_id: u64,
        amounts: Vec<U128>,
        min_amounts: Option<Vec<U128>>,
    ) -> U128;
}
#[ext_contract(ext_wnear)]
trait wnear {
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

pub struct AddLiquidity {
    poolId : u32,
}

impl AddLiquidity {
    pub fn new(pooldId: u32) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        AddLiquidity {
           poolId : pooldId,
        }
    }
    pub fn swap( account_id: ValidAccountId, amount : U128)  {
        let contractRef: AccountId = "ref-finance-101.testnet".parse().unwrap();
        let s :String =  format!("{{\"force\":0,\"actions\":[{{\"pool_id\":387,\"token_in\":\"wrap.testnet\",\"token_out\":\"token11.duyhuongtest.testnet\",\"amount_in\":\"10000000000000000000000\",\"min_amount_out\":\"82323310\"}}]}}");
        ext_wnear::ft_transfer_call(contractRef,U128(10000000000000000000000),None, s,"wrap.testnet".parse().unwrap(), 3000000000000000000000000,GAS);
    }
    pub fn add_liquidity (&mut self,amounts: Vec<U128>,min_amounts: Option<Vec<U128>>) {
        ext_ref::add_liquidity(self.poolId,amounts,min_amounts,"ref-finance-101.testnet".parse().unwrap(),3000000000000000000000000,GAS);
    }
}

