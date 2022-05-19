use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Balance, Gas, PanicOnDefault};

const GAS: Gas = Gas(5_000_000_000_000);
const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(90_000_000_000_000);
const ONE_YOCTO: Balance = 1;
const ADD_LIQUID_AND_STORAGE_DEPOSIT: Balance = 960_000_000_000_000_000_000;

#[ext_contract(ext_token)]
trait token {
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;
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

#[ext_contract(ext_ref)]
pub trait RefFinance {
    fn add_liquidity(
        &mut self,
        pool_id: u64,
        amounts: Vec<U128>,
    ) -> U128;
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pool_id: u32,
    token_id: AccountId,
    owner_id : AccountId,
    ref_contract : AccountId,
    wrap_id : AccountId
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(pool_id: u32, token: AccountId,owner_id :AccountId,ref_contract : AccountId,wrap_id : AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self { pool_id: pool_id, token_id: token, owner_id : owner_id , ref_contract : ref_contract,wrap_id : wrap_id}
    }
    #[payable]
    fn internal_swap(&mut self, amount: Balance) {
        let s :String =  format!("{{\"force\":0,\"actions\":[{{\"pool_id\":{},\"token_in\":\"{}\",\"token_out\":\"{}\",\"amount_in\":\"{}\",\"min_amount_out\":\"0\"}}]}}",self.pool_id,self.token_id,self.wrap_id,amount);
        ext_token::ft_transfer_call(
            self.ref_contract.clone(),
            U128(amount),
            None,
            s,
            self.token_id.clone(),
            ONE_YOCTO,
            GAS_FOR_FT_TRANSFER_CALL,
        );
    }
    #[payable]
    fn internal_add_liquidity(&mut self, amounts: Vec<U128>) {
       ext_wnear::ft_transfer_call(
        self.ref_contract.clone(),
        amounts[1],
        None,
        "".to_string(),
        self.wrap_id.clone(),
        ONE_YOCTO,
        GAS_FOR_FT_TRANSFER_CALL,
    );

        ext_token::ft_transfer_call(
            self.ref_contract.clone(),
            amounts[0],
            None,
            "".to_string(),
            self.token_id.clone(),
            ONE_YOCTO,
            GAS_FOR_FT_TRANSFER_CALL,
        );
       

        ext_ref::add_liquidity(
            self.pool_id.into(),
            amounts,
            self.ref_contract.clone(),
            ADD_LIQUID_AND_STORAGE_DEPOSIT,
            GAS,
        );
    }
    #[payable]
    pub fn swapAndLiquidify(&mut self, amountSwap: Balance, amounts: Vec<U128>) {
        self.assert_owner();
        self.internal_swap(amountSwap);

        self.internal_add_liquidity(amounts);
    }
    #[payable]
    pub fn set_owner(&mut self, owner_id: AccountId) {
        assert_eq!(env::attached_deposit(), 1, "Requires attached deposit of exactly 1 yoctoNEAR");      
        self.assert_owner();
        self.owner_id = owner_id;
    }
    #[payable]
    pub fn set_pool_id(&mut self, pool_id: u32) {
        assert_eq!(env::attached_deposit(), 1, "Requires attached deposit of exactly 1 yoctoNEAR");      
        self.assert_owner();
        self.pool_id = pool_id;
    }
    #[payable]
    pub fn set_token_id(&mut self, token_id: AccountId) {
        assert_eq!(env::attached_deposit(), 1, "Requires attached deposit of exactly 1 yoctoNEAR");      
        self.assert_owner();
        self.token_id = token_id;
    }
    #[payable]
    pub fn set_ref_contract(&mut self, ref_contract: AccountId) {
        assert_eq!(env::attached_deposit(), 1, "Requires attached deposit of exactly 1 yoctoNEAR");      
        self.assert_owner();
        self.ref_contract = ref_contract;
    }

     fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id ,
            "{}", "no permission to invoke this"
        );
    }

}
