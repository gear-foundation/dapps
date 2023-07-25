import { Navigate, Route, Routes, useLocation } from 'react-router-dom'
import { lazy, Suspense } from 'react'
import { useWalletSync } from '@/features/wallet/hooks'
import { useAuth, useAuthSync } from '@/features/auth'
import { ROUTES } from '@/app/consts'
import { Loader } from '@/components'
import { useAccount } from '@gear-js/react-hooks'
import { useInitBattleData } from '@/features/battle-tamagotchi/hooks'

function RequireAuth({ children }: { children: JSX.Element }) {
  const { authToken } = useAuth()
  const { account } = useAccount()
  const location = useLocation()

  if (!authToken && account) {
    return <Navigate to={ROUTES.UNAUTHORIZED} replace />
  }

  if (!authToken) {
    // Redirect them to the /login page, but save the current location they were
    // trying to go to when they were redirected. This allows us to send them
    // along to that page after they log in, which is a nicer user experience
    // than dropping them off on the home page.
    return <Navigate to={ROUTES.LOGIN} state={{ from: location }} replace />
  }

  return children
}

const Game = lazy(() => import('./home'))
const Login = lazy(() => import('./login'))
const NotAuthorized = lazy(() => import('./not-authorized'))
const Leaderboard = lazy(() => import('./leaderboard'))
const NotFound = lazy(() => import('./not-found'))

const routes = [
  { path: ROUTES.LOGIN, Page: Login },
  { path: ROUTES.HOME, Page: Game, isPrivate: true },
  { path: ROUTES.UNAUTHORIZED, Page: NotAuthorized },
  { path: ROUTES.LEADERBOARD, Page: Leaderboard },
  { path: ROUTES.NOTFOUND, Page: NotFound },
]

export const Routing = () => {
  // should be executed after loaders, cuz of useDidUpdate effect with isAccountReady check
  useWalletSync()
  useAuthSync()
  useInitBattleData()

  return (
    <Routes>
      {routes.map(({ path, Page, isPrivate }) => (
        <Route
          key={path}
          path={path}
          element={
            <Suspense fallback={<Loader />}>
              {isPrivate ? (
                <RequireAuth>
                  <Page />
                </RequireAuth>
              ) : (
                <Page />
              )}
            </Suspense>
          }
        />
      ))}
    </Routes>
  )
}
