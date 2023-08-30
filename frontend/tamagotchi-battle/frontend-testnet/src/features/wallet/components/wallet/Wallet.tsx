import { lazy, useState } from 'react'
import { Account } from '@gear-js/react-hooks'
import { AnimatePresence } from 'framer-motion'
import { Button } from '@/components/ui/button'
import styles from './Wallet.module.scss'
import { WalletModal } from '../wallet-modal'

const Identicon = lazy(() => import('@polkadot/react-identicon'))

export function Wallet({
  account,
  isReady = true,
}: {
  account: Account
  isReady?: boolean
}) {
  const [isOpen, setIsOpen] = useState(false)

  const openWallet = () => setIsOpen(true)
  const closeWallet = () => setIsOpen(false)

  return (
    <div>
      <Button
        variant={isReady && account ? 'black' : 'white'}
        className={styles.button}
        onClick={openWallet}
        disabled={!isReady}
      >
        {isReady && account && (
          <Identicon
            value={account.address}
            size={16}
            theme="polkadot"
            className={styles.icon}
          />
        )}
        <span>{isReady && account ? account.meta.name : 'Connect Wallet'}</span>
      </Button>

      <AnimatePresence>
        {isOpen && <WalletModal onClose={closeWallet} />}
      </AnimatePresence>
    </div>
  )
}
