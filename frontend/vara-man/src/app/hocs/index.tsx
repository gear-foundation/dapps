import type { ComponentType } from 'react'
import { BrowserRouter } from 'react-router-dom'
import {
  ApiProvider as GearApiProvider,
  AlertProvider as GearAlertProvider,
  AccountProvider,
  ProviderProps,
} from '@gear-js/react-hooks'
import { Alert, alertStyles } from '@gear-js/ui'
import { ENV } from '@/app/consts'
import { AppProvider } from '@/app/context/ctx-app'
import { GameProvider } from '@/app/context/ctx-game'

const ApiProvider = ({ children }: ProviderProps) => (
  <GearApiProvider providerAddress={ENV.NODE}>{children}</GearApiProvider>
)

const AlertProvider = ({ children }: ProviderProps) => (
  <GearAlertProvider template={Alert} containerClassName={alertStyles.root}>
    {children}
  </GearAlertProvider>
)

const BrowserRouterProvider = ({ children }: ProviderProps) => (
  <BrowserRouter>{children}</BrowserRouter>
)

const providers = [
  BrowserRouterProvider,
  AlertProvider,
  ApiProvider,
  AccountProvider,
  AppProvider,
  GameProvider,
]

export const withProviders = (Component: ComponentType) => () =>
  providers.reduceRight(
    (children, Provider) => <Provider>{children}</Provider>,
    <Component />
  )
