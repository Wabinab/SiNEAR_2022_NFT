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

- speak about wallet both can see. 
- dropped approvals and royalty

- Image needs URL not upload function: it's not the best, but oh well, not everyone build the best anyways! 

## References
- https://docs.near.org/docs/tutorials/contracts/nfts/introduction
- https://github.com/near-examples/nft-tutorial
