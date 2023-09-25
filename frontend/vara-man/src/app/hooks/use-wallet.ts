import { useState } from 'react'
import { LOCAL_STORAGE } from '@/app/consts'
import { useAccount } from '@gear-js/react-hooks'
import {
  EnkryptSVG,
  PolkadotSVG,
  SubWalletSVG,
  TalismanSVG,
} from '@/assets/images/wallet'

const WALLET = {
  'polkadot-js': { name: 'Polkadot JS', SVG: PolkadotSVG },
  'subwallet-js': { name: 'SubWallet', SVG: SubWalletSVG },
  talisman: { name: 'Talisman', SVG: TalismanSVG },
  enkrypt: { name: 'Enkrypt', SVG: EnkryptSVG },
}

type WalletId = keyof typeof WALLET

function useWallet() {
  const { accounts } = useAccount()

  const [walletId, setWalletId] = useState<WalletId | undefined>(
    localStorage[LOCAL_STORAGE.WALLET]
  )

  const resetWalletId = () => setWalletId(undefined)

  const getWalletAccounts = (id: WalletId) =>
    accounts.filter(({ meta }) => meta.source === id)

  const saveWallet = () =>
    walletId && localStorage.setItem(LOCAL_STORAGE.WALLET, walletId)

  const removeWallet = () => localStorage.removeItem(LOCAL_STORAGE.WALLET)

  const wallet = walletId && WALLET[walletId]
  const walletAccounts = walletId && getWalletAccounts(walletId)

  return {
    wallet,
    walletAccounts,
    setWalletId,
    resetWalletId,
    getWalletAccounts,
    saveWallet,
    removeWallet,
  }
}

export { useWallet }
