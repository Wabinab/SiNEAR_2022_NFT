use crate::*;
use near_sdk::{ext_contract, Gas, PromiseResult, require};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;


pub trait NonFungibleTokenCore {
    /// Transfers an NFT to a receiver ID
    fn nft_transfer(
      &mut self,
      receiver_id: AccountId,
      percentage: Percentage,
      token_id: TokenId,
      approval_id: u64,
      memo: Option<String>,
    );

    /// Transfers an NFT to a receiver and calls a function on the receiver ID's 
    /// contract. 
    /// Returns `true` if the token was transferred from the sender's account.
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        percentage: Percentage,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool>;

    /// Get information about the NFT token passed in
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;
}


#[ext_contract(ext_non_fungible_token_receiver)]
trait NonFungibleTokenReceiver {
    /// Returns `true` if the token should be returned back to the sender.
    fn nft_on_transfer(
      &mut self,
      sender_id: AccountId,
      previous_owner_id: AccountId,
      percentage: Percentage,
      token_id: TokenId,
      msg: String,
    ) -> Promise;
}

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    /*
        resolves the promise of the cross contract call to the receiver contract
        this is stored on THIS contract and is meant to analyze what happened in the cross contract call when nft_on_transfer was called
        as part of the nft_transfer_call method
    */
    fn nft_resolve_transfer(
      &mut self,
      authorized_id: Option<String>,
      owner_id: AccountId,
      receiver_id: AccountId,
      percentage: Percentage,
      previous_receiver_percentage: Percentage,
      token_id: TokenId,
      // approved_account_ids: HashMap<AccountId, u64>,
      all_owners: HashMap<AccountId, Percentage>,
      memo: Option<String>,
    ) -> bool;
}


/*
    resolves the promise of the cross contract call to the receiver contract
    this is stored on THIS contract and is meant to analyze what happened in the cross contract call when nft_on_transfer was called
    as part of the nft_transfer_call method
*/ 
trait NonFungibleTokenResolver {
  fn nft_resolve_transfer(
    &mut self,
    authorized_id: Option<String>,
    owner_id: AccountId,
    receiver_id: AccountId,
    percentage: Percentage,
    previous_receiver_percentage: Percentage,
    token_id: TokenId,
    // approved_account_ids: HashMap<AccountId, u64>,
    all_owners: HashMap<AccountId, Percentage>,
    memo: Option<String>,
  ) -> bool;
}


#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    /// Implementation of the nft_transfer method. This transfers the NFT from 
    /// the current owner to the receiver. 
    #[payable]
    fn nft_transfer(
      &mut self,
      receiver_id: AccountId,
      percentage: Percentage,
      token_id: TokenId,
      approval_id: u64,
      memo: Option<String>,
    ) {
      // assert user attached exactly 1 yoctoNEAR. This is for security
      // and that the user will be redirected to the NEAR wallet. 
      assert_one_yocto();

      // get sender to transfer token from sender to receiver. 
      let sender_id = env::predecessor_account_id();

      // If we have approval, we need to get signer_account_id() in the future. 

      // call the internal transfer method
      // return previous token so we can refund the approved account IDs. 
      let (_previous_token, current_token) = self.internal_transfer(
        &sender_id,
        &receiver_id,
        &percentage,
        &token_id,
        Some(approval_id),
        memo,
      );

      // refund owner for releasing the used up storage by approved account IDs.
      // refund_approved_account_ids(
      //   // previous_token.owner_id.clone(),
      //   sender_id.clone(),
      //   &previous_token.approved_account_ids,
      // );

      if let None = current_token.all_owners.get(&sender_id) {
        refund_if_empty(sender_id);
      }
    }

    /// Implementation of the transfer call method. This will transfer the NFT 
    /// and call a method on the receiver_id contract.
    #[payable]
    fn nft_transfer_call(
      &mut self,
      receiver_id: AccountId,
      percentage: Percentage,
      token_id: TokenId,
      approval_id: u64,
      memo: Option<String>,
      msg: String,
    ) -> PromiseOrValue<bool> {
      assert_one_yocto();

      // assert enough GAS
      let attached_gas = env::prepaid_gas();
      require!(
        attached_gas >= MIN_GAS_FOR_NFT_TRANSFER_CALL,
        format!(
          "You cannot attach less than {:?} Gas to nft_transfer_call",
          MIN_GAS_FOR_NFT_TRANSFER_CALL
        ),
      );

      // Currently w/o approval, sender MUST HOLD A SHARE to the F-NFT. 
      // Hence sender_id == previous_owner of token. At least for now. 
      let sender_id = env::predecessor_account_id();

      // transfer token and get previous token object
      let (previous_token, current_token) = self.internal_transfer(
        &sender_id,
        &receiver_id,
        &percentage,
        &token_id,
        Some(approval_id),
        memo.clone(),
      );

      // we now need authorized ID to be passed in to 
      // function. 
      // let mut authorized_id = None;

      // if sender not owner of token, set authorized ID = sender.
      // if sender_id != previous_token.owner_id {
      //   authorized_id = Some(sender_id.to_string());
      // }

      // We just let authorized_id equal sender id immediately. 
      let authorized_id = Some(sender_id.to_string());

      let previous_percentage: Percentage = match previous_token.all_owners.get(&receiver_id) {
        Some(percentage) => *percentage,
        None => 0u16
      };
      
      // initiating receiver's call and callback
      ext_non_fungible_token_receiver::nft_on_transfer(
        sender_id.clone(),
        // previous_token.owner_id.clone(),
        sender_id.clone(),
        percentage.clone(),
        token_id.clone(),
        msg,
        receiver_id.clone(),  // contract account to make the call to
        NO_DEPOSIT,  // attached deposit
        env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL,  // attached gas.
      ).then(
        ext_self::nft_resolve_transfer(
          authorized_id,
          // previous_token.owner_id,
          sender_id,
          receiver_id,
          percentage,
          previous_percentage,
          token_id,
          // previous_token.approved_account_ids,
          current_token.all_owners,
          memo,
          env::current_account_id(),
          NO_DEPOSIT,
          GAS_FOR_RESOLVE_TRANSFER,
        )
      ).into()

    }


    /// Get the information for a specific token ID
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
      if let Some(token) = self.tokens_by_id.get(&token_id) {
        let metadata = self.token_metadata_by_id.get(&token_id).unwrap();

        Some(JsonToken {
          token_id,
          metadata,
          approved_account_ids: token.approved_account_ids,
          all_owners: token.all_owners
        })
      } else {
        None
      }
    }
}


#[near_bindgen]
impl NonFungibleTokenResolver for Contract {
    /// Resolves the cross contract call when calling nft_on_transfer in the 
    /// nft_transfer_call method. 
    /// Returns true if the token was successfully transferred to the receiver_id. 
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        percentage: Percentage,
        previous_percentage: Percentage,
        token_id: TokenId,
        // approved_account_ids: HashMap<AccountId, u64>,
        all_owners: HashMap<AccountId, Percentage>,
        memo: Option<String>,  // for logging transfer event. 
    ) -> bool {
      if let PromiseResult::Successful(value) = env::promise_result(0) {
        if let Ok(return_token) = near_sdk::serde_json::from_slice::<bool>(&value) {
          // don't need to return token, simply return true. 
          // everything went fine. 
          if !return_token {
            // Since we've alredy transferred token and nft_on_transfer returns false,
            // we don't have to revert the original transfer, thus we can just return
            // true since nothing went wrong. We refund the owner for releasing the
            // storage used up by the approved account IDs. 
            // refund_approved_account_ids(owner_id, &approved_account_ids);

            if let None = all_owners.get(&owner_id) {
              refund_if_empty(owner_id);
            }
            return true;
          }
        }
      }

      // get token object if got some token object
      let mut token = if let Some(token) = self.tokens_by_id.get(&token_id) {
        // I have no idea about this logic. 
        // if token.owner_id != receiver_id {  // receiver_id is the receiver. 
        //   refund_approved_account_ids(owner_id, &approved_account_ids);
        //   return true;  
        // }

        // Instead of checking for owner_id, we check if the receiver_id is NOT inside
        // all_owners
        if let Some(current_percentage) = token.all_owners.get(&receiver_id) {
          // refund_approved_account_ids(owner_id, &approved_account_ids);

          // Check correct percentage. Ultimately, the percentage changes, 
          if current_percentage == &(previous_percentage + percentage) {
            refund_if_empty(owner_id);
            return true;
          }
        }
        token
      } else {  // no token object, it was burned. 
        // refund_approved_account_ids(owner_id, &approved_account_ids);
        refund_if_empty(owner_id);
        return true;
      };

      env::log_str(
        format!(
          "Return {} from @{} to @{}, percentage {}",
          token_id,
          receiver_id,
          owner_id,
          percentage as f32 / 100f32   
        ).as_str(),
      );

      let percentage_value = token.all_owners.get(&owner_id).unwrap();

      self.internal_remove_token_from_owner(&owner_id, &token_id);

      if percentage_value == &percentage {
        // remove token from it's current owner's set
        self.internal_add_token_to_owner(&owner_id, &token_id);
        token.all_owners.remove(&receiver_id);
      } else {
        // Get percentage original owner has and add it back. 
        let current_percentage = token.all_owners.get(&owner_id).unwrap();
        let new_percentage = current_percentage + percentage;

        *token.all_owners.get_mut(&owner_id).unwrap() = new_percentage;
      }

      // Change the token struct's owner to original owner
      // token.owner_id = owner_id.clone();


      // Refund approved account IDs may have set on token. 
      // refund_approved_account_ids(receiver_id.clone(), &token.approved_account_ids);
      refund_if_empty(receiver_id.clone());

      // we insert the token back into the tokens_by_id collection
      self.tokens_by_id.insert(&token_id, &token);

      // log reverted NFT transfer. 
      let nft_transfer_log: EventLog = EventLog {
        standard: NFT_STANDARD_NAME.to_string(),
        version : NFT_METADATA_SPEC.to_string(),
        event   : EventLogVariant::NftTransfer(vec![NftTransferLog {
          authorized_id,
          old_owner_id: receiver_id.to_string(),
          new_owner_id: owner_id.to_string(),
          percentage_new_owner: 10u16,  // to be changed. 
          token_ids: vec![token_id.to_string()],
          memo,
        }]),
      };

      env::log_str(&nft_transfer_log.to_string());

      // receiver_id didn't successfully receive the token. 
      false
    }
}
