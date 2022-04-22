use crate::*;

pub const MAX_GAS: u64 = 300_000_000_000_000;
pub const MIN_BALANCE_FOR_STORAGE: u128 = 1_000_000_000_000_000_000_000_000;
pub const CONTRACT_ACCOUNT_ID: &str = "fnft";


pub(crate) fn basic_setup() -> (UserAccount, UserAccount, UserAccount, UserAccount) {
    let mut genesis_config = GenesisConfig::default();
    genesis_config.block_prod_time = 0;
    let root = init_simulator(Some(genesis_config));

    // The first person minting the NFT. 
    // Imitating testnet, we give each person 200 NEAR. 
    let minter = root.create_user(
      "minter".parse().unwrap(), 
      to_yocto("200")
    );

    // Define two other persons to transfer between each other. 
    let alice = root.create_user(
      "alice".parse().unwrap(), 
      to_yocto("200")
    );

    let bob = root.create_user(
      "bob".parse().unwrap(), 
      to_yocto("200")
    );

    (root, minter, alice, bob)
}


pub(crate) fn basic_token_metadata() -> TokenMetadata {
  TokenMetadata {
    title: Some("F-NFT".to_owned()),
    description: Some("F-NFT test token".to_owned()),
    media: Some("https://www.google.com".to_owned()),  // doesn't matter
    media_hash: None,
    copies: None,

    issued_at: None,
    expires_at: None,
    starts_at: None,
    updated_at: None,
    extra: None,
    reference: None,
    reference_hash: None
  }
}


