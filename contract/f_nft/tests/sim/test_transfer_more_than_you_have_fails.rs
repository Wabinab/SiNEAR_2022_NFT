use crate::*;

// Because we use env::panic_str, it WILL NOT PANIC! 

#[test]
// #[should_panic(expected = "You try to send 150% of this NFT when you only have 100% of it.")]
fn transfer_more_than_you_have_fails() {
    let contract_amount = to_yocto("1000");
    let (root, minter, alice, _bob) = basic_setup();

    let contract = deploy!(
      contract: ContractContract,
      contract_id: CONTRACT_ACCOUNT_ID.to_string(),
      bytes: &FNFT_WASM_BYTES,
      signer_account: root, 
      deposit: MIN_BALANCE_FOR_STORAGE + contract_amount,
      gas: MAX_GAS,
      init_method: new_default_meta(
        root.account_id.clone()  // is it? if not, use minter. 
      )
    );

    // ======================== CLOSURES ============================

    let get_tokens_for_owner = |account_id: AccountId| -> Vec<JsonToken> {
      minter.view_method_call(
        contract.contract.nft_tokens_for_owner(
          account_id,
          None, 
          Some(10)
        )
      ).unwrap_json()
    };

    // ==============================================================

    let token_id: &str = "token-1";

    minter.function_call(
      contract.contract.nft_mint(
        token_id.to_owned(),
        basic_token_metadata(),
        minter.account_id.clone()
      ),
      MAX_GAS,
      to_yocto("0.1")
    );

    let transfer_percentage: Percentage = 15000;  // 150% ???
    let approval_id: u64 = 1;  // redundant, we didn't remove as approval may be added back. 

    minter.function_call(
      contract.contract.nft_transfer(
        alice.account_id.clone(),
        transfer_percentage.clone(),
        token_id.to_owned(),
        approval_id.clone(),
        Some("memo".to_owned())
      ),
      MAX_GAS,
      1u128  // 1 yoctoNear. 
    );
    
    // So instead of panicking, we'll check that the original owner, "minter", still
    // have all of it. We also check that alice have no share. 
    let res: Vec<JsonToken> = get_tokens_for_owner(minter.account_id.clone());
    assert_eq!(
      res[0].all_owners.get(&minter.account_id).unwrap_or(&0u16), 
      &10000u16,
      "Original owner have wrong percentage of NFT share."
    );

    let res: Vec<JsonToken> = get_tokens_for_owner(alice.account_id.clone());
    assert_eq!(res.len(), 0);  // it doesn't even exist. 

    // eprintln!("{:#?}", res);
}