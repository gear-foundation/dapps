import { Link } from 'react-router-dom'
import { useGame } from '@/app/context/ctx-game'
import { HeaderAdmin } from '@/components/layout/header/header-admin'
import { Icons } from '@/components/ui/icons'

import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui'
import {
	EzGaslessTransactions,
	EzSignlessTransactions,
	useEzTransactions,
} from '@dapps-frontend/ez-transactions'
import { useAccount, useAlert } from '@gear-js/react-hooks'

import styles from './Header.module.scss'
import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts'

export const Header = () => {
	const { isAdmin } = useGame()
	const { account } = useAccount()
	const { gasless, signless } = useEzTransactions()
	const alert = useAlert()

	return (
		<CommonHeader
			logo={
				<Link to="/">
					<Icons.logo className="h-15" />
				</Link>
			}
			menu={
				<MenuHandler
					className={{
						wallet: {
							balance: styles.walletBalance,
						},
						icon: styles.menuIcon,
					}}
					customItems={[
						{ key: 'signless', option: <EzSignlessTransactions allowedActions={SIGNLESS_ALLOWED_ACTIONS}/> },
						{ key: 'gasless', option: <EzGaslessTransactions /> },
					]}
				/>
			}
			className={{ header: styles.header, content: styles.container }}
		>
			{isAdmin && <HeaderAdmin />}
		</CommonHeader>
	)
}
