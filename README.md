# Spring is NEAR Fractionalized-NFTs

The first thing you did with this, is to get your environmental keys setup. 
For security reasons, rails by default do not export these, and requires you
to deal with it individually. You don't need my keys to play with this example
anyways, so one shall give you instruction in generating new keys. 

Based on [this article](https://stackoverflow.com/questions/54277392/rails-activesupportmessageencryptorinvalidmessage), do these: 

Delete these if exist (usually it don't if you clone from github):
- config/master.key
- config/credentials.yml.enc

provided you're using **VSCode**:

```bash
EDITOR="code ." bin/rails credentials:edit
```

This will open up a file. DOn't need to touch it, just close it. If it asks for saving, ignore it. Most probably you typed a space or soemthing which isn't important. 

Try run this: 

```bash
rails s
```

If it opens the browser and asks you to do migration, do these. With the new key, you should be able to run these without getting into `Rails:ActiveSupport::MessageEncryptor::InvalidMessage` error. 

```bash
rails db:migrate
rails s
```

> `rails s` is equivalent to `rails server`. 

It should work now... 

# Introductions; Explanations
## Contract

As usual, contract is in `contract` folder. One isn't sure if there's F-NFTs in NEAR github or not, at least one do not refer to anything nor do research on it and alter this based on just the NFT tutorial github page. Irregardless, we remove quite a lot of functions, including `approvals` and `royalty` (and of course, `marketplace`) that are available in the original tutorial. 
With these removals, something that is so complicated, trying to solve from various perspective, is simplified. We don't need to worry how to approve others, we don't need to care about royalties, and we don't need to care about marketplace. 

Then, based on how `royalty` is implemented, we kinda copy its implementation to implement `all_owners`. Nothing too complicated! 

Just to ensure they are correct, we wrote some simulation tests, which can be found in `tests/sim` folder. These didn't cover everything for sure, like some of the callbacks responsible for returning storage after freeing them up isn't totally accounted for as one doesn't know how to really test these. 
We do not write any unit testing, one do not have time for that. Considerably, most unit test can be moved from original NFT's unit test to here, with small changes, and things should be fine. 
**Also note that, we don't unit-test F-NFT runs as expected. We do sim tests on that!** 
Hence, considering that our NFT code is correct (if there's unit test for it), then we can just go towards sim tests. We only test for the functions that we need for our frontend, not every single view and change method and their panics. Even so, we don't cover every single panic situation like passing in negative numbers, etc. 

Finally, the percentage is a `u16` value. Why `u16`, well, to not waste storage space! We only need 100.00% to 2 decimal places, which maximum is 10000, well within `u16`. The smallest share of the NFT you can get is 0.01%. 

That's about it for the contract. Let's look at frontend. 

## Frontend

(if we speak of "cards", we mean "nft")

As usual, we have some problem with saving users to database, so we'll do that. In contrary, one realized we can create cards static pages and render them without database! Perhaps we'll work on the user next time, maybe, if we have time. For now, just continue with login then click on a button to save info to database, and after clicking button redirect to page. If not, **the search bar on the right can search for user ID, BUT NOT NFT's!!!**. 
Searching for a specific account like "wabinab.testnet" takes you to the page, if you can't remember your page which looks like "/users/1". 

The cards do have a database, but we don't use it. 

If users don't have a database in the future, searching for it needs to be exact. 
Particularly, one don't know how the search function works online, otherwise, we could
have a popup at the bottom for suggestions, just like more user-friendly search functions 
on nearblocks.io. Ours is more like the explorer's search function. 
Anyway, without database, we could fetch any wallet and display them rather than like now, 
**if you don't register yourself, you can't display yourself.** This is a weakness of this
design. 

As usual, because change methods are on the frontend, (while view functions are on the backend), anything that runs on frontend cannot be pass to backend easily (it requires a complicated setup that one didn't make it work, unfortunately.), so if you click on buttons etc, there
would be no freezing of buttons, sometimes even don't have an alert reaction despite we coded one (because the page refreshes!), moving on... 

Minting is just usual stuffs. Owner mint, 100% of share to owner, no split mint (that's being too sarcastic). If you want 50% to others, mint and transfer! 

We don't do transfer button on users page, though. Just like NEAR wallet, you can only transfer after clicking into the NFT. Here, after clicking to NFT, it'll take you to the page where you see a button at the bottom to perform the transfer. Clicking on it will un-collapse the transfer form. 
**After transfer, please MANUALLY RELOAD THE PAGE to update the owner. That's a bug fixed (I think) in localhost but not after deployed.** Reloading the page, you can see the newly updated owners sharing the NFT. 

The creation of NFT isn't one click "open crate" (someone make the cat nft open crate which have an awesome frontend for this Spring is NEAR, if you'd seen it) and it assigns you image. Ours, you put in the title, description, and an image hyperlink (**must be direct to image**. If you upload to IPFS, make sure to get ipfs.io/CID/**image_name.png** instead of ipfs.io/CID, otherwise won't render!) Upon creation, you should see it in your page. 
If not, just refresh your user page. 

If two of you shares the nft, both of you can see the NFT on your user page. **In fact, both of you can see the same NFT in your NEAR web wallet!** The web wallet don't show the percentage of hold, but ours do! So that's a difference. 

It's not possible to transfer F-NFT from wallet, because we have a different interface than what it expects. Particularly, we do not make Percentage an `std::option::Option`, (it kinda complicates), so it doesn't work. We do not have a "max" for percentage to autofill, though. It's just an addition that one thought is optional to add, so "no" we goes. 

Overall, it's not the best, we can do better, but oh well, not everybody build the best anyways! 

Thanks for reading! 

## References
- https://docs.near.org/docs/tutorials/contracts/nfts/introduction
- https://github.com/near-examples/nft-tutorial
