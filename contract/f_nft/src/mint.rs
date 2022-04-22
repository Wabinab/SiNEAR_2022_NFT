use crate::*;
use near_sdk::require;


#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
      &mut self,
      token_id:TokenId,
      metadata: TokenMetadata,
      receiver_id: AccountId,
    ) {
      // measure the initial storage being used on contract.
      let initial_storage_usage = env::storage_usage();

      // create all_owners map to store the token.
      let mut all_owners = HashMap::new();

      // Owner currently has 100% of the data. 
      all_owners.insert(receiver_id.clone(), 100_00u16);

      // Might limit the number of owners for F-NFTs in the future? 
      // Don't know! 

      // specify the token struct that contains the owner ID. 
      let token = Token {
        // owner_id: receiver_id,
        approved_account_ids: Default::default(),  // default value is empty map.
        next_approval_id: 0,
        all_owners,
      };

      // insert token ID and token struct and make sure token
      // doesn't exist. 
      require!(
        self.tokens_by_id.insert(&token_id, &token).is_none(),
        "Token already exists."
      );

      self.token_metadata_by_id.insert(&token_id, &metadata);

      // Just for owner_id's sake: 
      let owner_id = receiver_id;

      self.internal_add_token_to_owner(&owner_id, &token_id);

      // Log the minting as per events standard. 
      // Minting still have "owner_id" because it's the first person whom mint. 
      let nft_mint_log: EventLog = EventLog {
        standard: NFT_STANDARD_NAME.to_string(),
        version : NFT_METADATA_SPEC.to_string(),
        event   : EventLogVariant::NftMint(vec![NftMintLog {
          owner_id : owner_id.to_string(),
          token_ids: vec![token_id.to_string()],
          memo     : None,  // optional
        }]),
      };

      // log serialized json
      env::log_str(&nft_mint_log.to_string());

      // calculate required storage
      let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

      // refund excess storage if user attached too much. 
      // Panic if they didn't attach enough. 
      refund_deposit(required_storage_in_bytes);
    }
}