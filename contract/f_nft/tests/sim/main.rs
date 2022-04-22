use f_nft::{
  ContractContract, TokenMetadata, JsonToken, Percentage
};

use near_sdk::AccountId;

use near_sdk_sim::runtime::GenesisConfig;
use near_sdk_sim::{init_simulator, to_yocto, UserAccount};
use near_sdk_sim::deploy;

pub(crate) use utils::*;
mod utils;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
  FNFT_WASM_BYTES => "res/f_nft.wasm"
}


mod test_transfer_as_expected;
mod test_transfer_more_than_you_have_fails;
