import { Route, Routes } from 'react-router-dom'
import { ADDRESS, ROUTES } from '@/app/consts'
import * as Sentry from '@sentry/react'
import Home from '@/pages/home'
import NotFoundPage from '@/pages/not-found'

const SafeRoutes = ADDRESS.SENTRY_DSN
  ? Sentry.withSentryReactRouterV6Routing(Routes)
  : Routes

const routes = [
  { path: ROUTES.HOME, Page: Home },
  { path: ROUTES.NOTFOUND, Page: NotFoundPage },
]

export function Routing() {
  return (
    <SafeRoutes>
      {routes.map(({ path, Page }) => (
        <Route key={path} path={path} element={<Page />} />
      ))}
    </SafeRoutes>
  )
}
