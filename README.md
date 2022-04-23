# Spring Is NEAR Greeter

Spring is NEAR greeter contract. Enter the name, it call the smart contract and will greet you. 

The smart contract is located in the `contract` folder. The rest are responsible for frontend code; built with Ruby on Rails. 

To run localhost: In command prompt:

```bash
rails server
```

Smart contract is deployed separately from frontend. To deploy, go to `contract/greeter/build.sh` and change all `wabinab.testnet` occurrences to your testnet account; install wasm-opt from https://github.com/WebAssembly/binaryen/releases/tag/version_105 (unzip and add its `bin` folder to PATH). [References](https://github.com/WebAssembly/binaryen/discussions/3797)

Then run

```bash
bash contract/greeter/redeploy.sh
```

The first run you'll have an error `KeyNotFound` for the first command. That's **safe to ignore** (it's designated for subsequent redeployment anyways). 

Remember to change the frontend contract to redirect to your contract (and one mine) if you're calling to your subaccount/account. 

# Learnings

One actually learn lots of things! Things don't work as one expected them to, but oh well... 

And one is still learning how to write in Ruby on Rails... 

### References
- `npx create-app --contract=Rust` for Greeter app in Rust; but we upgrade it to v4.0.0-pre.4. 
- https://read.cash/@wabinab/near-api-js-with-rails-7-c749d832
