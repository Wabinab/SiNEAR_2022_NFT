# README

## How it goes
- Minting can only be done by a single owner, just like usual NFT. 
- After minting, the NFT can be split into F-NFTs and distribute to owners. This works just 
like NFTs, but instead of transferring the whole NFTs, we just transfer a fraction of it. 
- Royalty implementation may be postponed. 

Each F-NFT have its own `owner_id`; but we also have `all_owners_id` in Contract to keep track of them. This is just like `approved_account_ids` which have not much use except to tell how many percentage of that NFT is being held. 

### Challenge #3. NFT+frontend
This is a 2-step challenge for minting your first NFT on NEAR and creating a frontend for it. It can be as simple or complex as you like! 

Step 1.  
Deploy an NFT smart contract on the testnet. Mint an NFT.

Step 2.  
Build a frontend to connect with the NFT smart contract you deployed (GitHub pages is the most simple option). The frontend should allow a user to log in with NEAR and mint an NFT to their own wallet. 

Share the link to 🧠│nearsping-submissions-week2  for us to review.


## References
- https://github.com/near-examples/nft-tutorial/blob/main/nft-contract/

