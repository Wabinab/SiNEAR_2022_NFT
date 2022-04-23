import { connect, Contract, keyStores, WalletConnection, utils } from 'near-api-js';
import getConfig from './config.js';


const nearConfig = getConfig('development', 'f_nft.wabinab.testnet')
const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig));

window.nearConfig = nearConfig
window.near = near

window.walletConnection = new WalletConnection(near)

window.accountId = window.walletConnection.getAccountId()

window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
  // View methods are read only. They don't modify the state, but usually return some value.
  // viewMethods: ['get_greeting'],
  // Change methods can modify the state. But you don't receive the returned value when called.
  changeMethods: ['nft_mint', 'nft_transfer'],
})


function logout() {
  window.walletConnection.signOut()
  window.location.replace(window.location.origin + window.location.pathname)
}

function login() {
  window.walletConnection.requestSignIn(nearConfig.contractName)
}


function check_id() {
  alert(window.walletConnection.getAccountId());
}


function nft_mint(token_id) {
  var title = document.getElementById("card_title").value;
  var description = document.getElementById("card_desc").value;
  var img_url = document.getElementById("card_img").value;

  window.contract.nft_mint(
    {
      "token_id": token_id,
      "metadata": {
        "title": title,
        "description": description,
        "media": img_url,
        "issued_at": Math.floor(Date.now() / 1000)
      },
      "receiver_id": window.walletConnection.getAccountId()
    }, 
    "30000000000000",  // 30 TGas
    utils.format.parseNearAmount("0.1")
  ).then(
    value => {
      alert("Minted. Click on specific nft to check that out.");
      window.location.reload();
    },
    err => alert(err),
  );  
}



window.nft_mint = nft_mint
window.check_id = check_id
window.logout = logout
window.login = login