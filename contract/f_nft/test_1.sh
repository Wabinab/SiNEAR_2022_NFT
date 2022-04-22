#!/bin/bash

export CONTRACT=f_nft.wabinab.testnet
export NFT_CONTRACT_ID=$CONTRACT

near call $NFT_CONTRACT_ID nft_mint '{
  "token_id": "'$1'", 
  "metadata": {
    "title": "My Non Fungible Team Token", 
    "description": "The Team Most Certainly Goes :)", 
    "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"
  }, 
  "receiver_id": "'$NFT_CONTRACT_ID'"
}' --accountId $NFT_CONTRACT_ID --amount 0.1

# near call $CONTRACT nft_approve '{
#   "token_id": "'$1'",
#   "account_id": "'$CONTRACT'"
# }' --accountId $CONTRACT --deposit 0.1

near call $CONTRACT nft_transfer '{
  "receiver_id": "wabinab.testnet",
  "token_id": "'$1'",
  "percentage": 15000,
  "approval_id": 1
}'  --accountId $CONTRACT --depositYocto 1


near view $CONTRACT nft_tokens_for_owner '{
  "account_id": "wabinab.testnet",
  "limit": 10
}'

# ===========================================
# near call $CONTRACT nft_transfer '{
#   "receiver_id": "somebodyelse.testnet",
#   "token_id": "'$1'",
#   "percentage": 2000,
#   "approval_id": 1
# }'  --accountId $CONTRACT --depositYocto 1

# near view $CONTRACT nft_tokens_for_owner '{
#   "account_id": "somebodyelse.testnet",
#   "limit": 10
# }'

# # ===========================================
# near call $CONTRACT nft_transfer '{
#   "receiver_id": "somebodyelse.testnet",
#   "token_id": "'$1'",
#   "percentage": 1000,
#   "approval_id": 1
# }'  --accountId wabinab.testnet --depositYocto 1

# near view $CONTRACT nft_tokens_for_owner '{
#   "account_id": "somebodyelse.testnet",
#   "limit": 10
# }'