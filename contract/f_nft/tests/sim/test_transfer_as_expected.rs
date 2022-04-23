use crate::*;


#[test]
fn transfer_as_expected() {
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


    // ========================= Closures ==================================
    // As far as my programming knowledge limits oneself, we can only use
    // this for view functions. 
    // For change functions, we need to specify who sign the transaction,
    // and one doesn't know how to do variable.function_call(...) and pass
    // in a variable. 

    let get_tokens_for_owner = |account_id: AccountId| -> Vec<JsonToken> {
      minter.view_method_call(
        contract.contract.nft_tokens_for_owner(
          account_id,
          None, 
          Some(10)
        )
      ).unwrap_json()
    };

    // ======================================================================

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

    // Skip approval

    let res: Vec<JsonToken> = get_tokens_for_owner(minter.account_id.clone());

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].all_owners.get(&minter.account_id).unwrap_or(&0u16), &10000u16);
    assert_eq!(res[0].token_id, token_id.to_owned());

    // Transfer token to alice, 25%, so 2500
    let transfer_percentage: Percentage = 2500;
    let alice_percentage: Percentage = transfer_percentage.clone();
    let minter_percentage: Percentage = 10000u16 - transfer_percentage;
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

    // Check for minter first. 
    let res: Vec<JsonToken> = get_tokens_for_owner(minter.account_id.clone());
    assert_eq!(
      res[0].all_owners.get(&minter.account_id).unwrap_or(&0u16), 
      &minter_percentage,
      "Original owner have wrong percentage of NFT share."
    );

    // Then we check for alice. 
    let res: Vec<JsonToken> = get_tokens_for_owner(alice.account_id.clone());
    assert_eq!(
      res[0].all_owners.get(&alice.account_id).unwrap_or(&0u16), 
      &transfer_percentage,
      "Receiver has wrong percentage of NFT share."
    );

    // ==========================================================================
    // Assert that both alice and minter could transfer share to Bob. 
    let transfer_percentage: Percentage = 1000;
    let bob_percentage = transfer_percentage.clone();
    let minter_percentage: Percentage = minter_percentage - transfer_percentage;

    minter.function_call(
      contract.contract.nft_transfer(
        bob.account_id.clone(),
        transfer_percentage.clone(),
        token_id.to_owned(),
        approval_id.clone(),
        Some("memo".to_owned())
      ),
      MAX_GAS,
      1u128  // 1 yoctoNear. 
    );

    // Check for minter first. 
    let res: Vec<JsonToken> = get_tokens_for_owner(minter.account_id.clone());
    assert_eq!(
      res[0].all_owners.get(&minter.account_id).unwrap_or(&0u16), 
      &minter_percentage,
      "Original owner have wrong percentage of NFT share."
    );

    // Then we check for alice. 
    let res: Vec<JsonToken> = get_tokens_for_owner(bob.account_id.clone());
    assert_eq!(
      res[0].all_owners.get(&bob.account_id).unwrap_or(&0u16), 
      &transfer_percentage,
      "Receiver has wrong percentage of NFT share."
    );


    // Could have use mutable variable lol...
    let alice_percentage = alice_percentage - transfer_percentage;
    let bob_percentage = bob_percentage + transfer_percentage;

    alice.function_call(
      contract.contract.nft_transfer(
        bob.account_id.clone(),
        transfer_percentage.clone(),
        token_id.to_owned(),
        approval_id.clone(),
        Some("memo".to_owned())
      ),
      MAX_GAS,
      1u128  // 1 yoctoNear. 
    );

    let res: Vec<JsonToken> = get_tokens_for_owner(alice.account_id.clone());
    assert_eq!(
      res[0].all_owners.get(&alice.account_id).unwrap_or(&0u16), 
      &alice_percentage,
      "Alice have wrong percentage of NFT share."
    );

    // Then we check for alice. 
    let res: Vec<JsonToken> = get_tokens_for_owner(bob.account_id.clone());
    assert_eq!(
      res[0].all_owners.get(&bob.account_id).unwrap_or(&0u16), 
      &bob_percentage,
      "Bob has wrong percentage of NFT share."
    );

    // ====================================================================
    // Alice transfer everything to bob, she'll be totally removed from "all owners". 
    let bob_percentage = bob_percentage + alice_percentage;
    let transfer_percentage = alice_percentage;

    alice.function_call(
      contract.contract.nft_transfer(
        bob.account_id.clone(),
        transfer_percentage.clone(),
        token_id.to_owned(),
        approval_id.clone(),
        Some("memo".to_owned())
      ),
      MAX_GAS,
      1u128  // 1 yoctoNear. 
    );

    let res: Vec<JsonToken> = get_tokens_for_owner(bob.account_id.clone());
    assert_eq!(
      res[0].all_owners.get(&bob.account_id).unwrap_or(&0u16), 
      &bob_percentage,
      "Bob has wrong percentage of NFT share."
    );

    let res: Vec<JsonToken> = get_tokens_for_owner(alice.account_id.clone());
    assert_eq!(res.len(), 0);

}