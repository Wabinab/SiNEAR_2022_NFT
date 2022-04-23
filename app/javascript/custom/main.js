import { connect, Contract, keyStores, WalletConnection } from 'near-api-js';
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




window.logout = logout
window.login = login