use crate::*;

#[test]
fn mint_multiple_as_expected() {
    let contract_amount = to_yocto("1000");
    let (root, minter, alice, bob) = basic_setup();

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

    // ====================== CLOSURES =======================================
    let get_tokens_for_owner = |account_id: AccountId| -> Vec<JsonToken> {
      minter.view_method_call(
        contract.contract.nft_tokens_for_owner(
          account_id,
          None, 
          Some(10)
        )
      ).unwrap_json()
    };
    // =======================================================================

    minter.function_call(
      contract.contract.nft_mint(
        "token-1".to_owned(),
        basic_token_metadata(),
        minter.account_id.clone()
      ),
      MAX_GAS,
      to_yocto("0.1")
    );

    minter.function_call(
      contract.contract.nft_mint(
        "token-2".to_owned(),
        basic_token_metadata(),
        minter.account_id.clone()
      ),
      MAX_GAS,
      to_yocto("0.1")
    );

    minter.function_call(
      contract.contract.nft_mint(
        "token-3".to_owned(),
        basic_token_metadata(),
        minter.account_id.clone()
      ),
      MAX_GAS,
      to_yocto("0.1")
    );

    let res: Vec<JsonToken> = get_tokens_for_owner(minter.account_id.clone());
    assert_eq!(res.len(), 3);
}