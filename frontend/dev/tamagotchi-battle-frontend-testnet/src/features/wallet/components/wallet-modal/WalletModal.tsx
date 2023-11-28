import { lazy, Suspense } from 'react'
import { decodeAddress } from '@gear-js/api'
import { useAccount, useAlert } from '@gear-js/react-hooks'
import { Button } from '@gear-js/ui'
import { Button as VaraButton } from '@/components/ui/button'
import { CopyIcon, EditIcon, ExitIcon } from '../../assets'
import { WALLETS } from '../../consts'
import { useWallet } from '../../hooks'
import { WalletItem } from '../wallet-item'
import styles from './WalletModal.module.scss'
import { copyToClipboard } from '@/app/utils'
import { Modal } from '@/components'
import { useGame } from '@/features/tic-tac-toe/hooks'
import { useAuth } from '@/features/auth'
import { ScrollArea } from '@/components/ui/scroll-area/scroll-area'

const Identicon = lazy(() => import('@polkadot/react-identicon'))

type Props = {
  onClose(): void
}

function WalletModal({ onClose }: Props) {
  const alert = useAlert()
  const { extensions, account, accounts } = useAccount()
  const { resetGameState } = useGame()
  const { signIn, signOut } = useAuth()

  const {
    wallet,
    walletAccounts,
    setWalletId,
    resetWalletId,
    getWalletAccounts,
  } = useWallet()

  const getWallets = () =>
    WALLETS.map(([id, { SVG, name }]) => {
      const isEnabled = extensions.some((extension) => extension.name === id)
      const status = isEnabled ? 'Enabled' : 'Disabled'

      const accountsCount = getWalletAccounts(id).length
      const accountsStatus = `${accountsCount} ${
        accountsCount === 1 ? 'account' : 'accounts'
      }`

      const onClick = () => setWalletId(id)

      return (
        <li key={id}>
          <VaraButton
            variant="white"
            className={styles.walletButton}
            onClick={onClick}
            disabled={!isEnabled}
          >
            <WalletItem icon={SVG} name={name} />

            <span className={styles.status}>
              <span className={styles.statusText}>{status}</span>

              {isEnabled && (
                <span className={styles.statusAccounts}>{accountsStatus}</span>
              )}
            </span>
          </VaraButton>
        </li>
      )
    })

  const getAccounts = () =>
    walletAccounts?.map((_account) => {
      const { address, meta } = _account

      const isActive = address === account?.address

      const handleClick = async () => {
        await signIn(_account)
        onClose()
      }

      const handleCopyClick = () => {
        const decodedAddress = decodeAddress(address)
        copyToClipboard({ value: decodedAddress, alert })
        onClose()
      }

      return (
        <li key={address}>
          <div className={styles.account}>
            <VaraButton
              variant={isActive ? 'primary' : 'white'}
              className={styles.accountButton}
              onClick={handleClick}
              disabled={isActive}
            >
              <Suspense>
                <Identicon
                  value={address}
                  size={20}
                  theme="polkadot"
                  className={styles.accountIcon}
                />
              </Suspense>
              <span>{meta.name}</span>
            </VaraButton>

            <VaraButton
              variant="text"
              className={styles.textButton}
              onClick={handleCopyClick}
            >
              <CopyIcon />
            </VaraButton>
          </div>
        </li>
      )
    })

  const handleLogoutButtonClick = () => {
    signOut()
    onClose()
    resetGameState()
  }

  return (
    <Modal heading="Wallet connection" onClose={onClose}>
      {accounts ? (
        <ScrollArea className={styles.content} type="auto">
          <ul className={styles.list}>{getAccounts() || getWallets()}</ul>
        </ScrollArea>
      ) : (
        <p>
          Polkadot extension was not found or disabled. Please,{' '}
          <a
            href="https://polkadot.js.org/extension/"
            target="_blank"
            rel="noreferrer"
          >
            install it
          </a>
          .
        </p>
      )}

      {wallet && (
        <div className={styles.footer}>
          <button
            type="button"
            className={styles.walletButton}
            onClick={resetWalletId}
          >
            <WalletItem icon={wallet.SVG} name={wallet.name} />

            <EditIcon />
          </button>

          {account && (
            <VaraButton
              variant="text"
              className={styles.textButton}
              onClick={handleLogoutButtonClick}
            >
              <ExitIcon />
              <span>Exit</span>
            </VaraButton>
          )}
        </div>
      )}
    </Modal>
  )
}

export { WalletModal }
