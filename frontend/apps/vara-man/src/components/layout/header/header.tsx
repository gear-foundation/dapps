import { Link } from 'react-router-dom'
import { useGame } from '@/app/context/ctx-game'
import { HeaderAdmin } from '@/components/layout/header/header-admin'
import { AccountComponent } from '@/components/layout/header/header-account'
import { HeaderLogo } from '@/components/layout/header/header-logo'
import { AccountGasBalance } from '@/components/common/account-gas-balance'
import { useAccount } from '@gear-js/react-hooks'
import { useApp } from '@/app/context/ctx-app'
import { Info } from 'lucide-react'

export const Header = () => {
  const { isSettled } = useApp()
  const { isAdmin } = useGame()
  const { account } = useAccount()

  return (
    <header className="container flex justify-between items-center py-7.5">
      <HeaderLogo />

      {account && isSettled && (
        <div className="flex space-x-4 ml-auto">
          {isAdmin ? (
            <HeaderAdmin />
          ) : (
            <>
              <Link
                to="/rules"
                className="btn space-x-2 bg-[#3081ED] px-6 hover:bg-blue-600 transition-colors"
              >
                <Info className="w-5 h-5" />
                <span>Show Rules</span>
              </Link>
              <AccountGasBalance />
            </>
          )}
        </div>
      )}

      <div className="ml-4">
        <AccountComponent />
      </div>
    </header>
  )
}
