import { AlertContainerFactory } from '@gear-js/react-hooks'
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types'
import { HexString } from '@polkadot/util/types'
import {
  NotificationResponseTypes,
  NotificationType,
  TamagotchiState,
} from '@/app/types/lessons'
import { LOCAL_STORAGE } from '@/app/consts'
import type {
  ItemsStoreResponse,
  StoreItemsNames,
  StoreItemType,
} from '@/app/types/ft-store'
import { ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

export const copyToClipboard = async (
  key: string,
  alert: AlertContainerFactory,
  successfulText?: string
) => {
  function unsecuredCopyToClipboard(text: string) {
    const textArea = document.createElement('textarea')
    textArea.value = text
    document.body.appendChild(textArea)
    textArea.focus()
    textArea.select()
    try {
      document.execCommand('copy')
      alert.success(successfulText || 'Copied')
    } catch (err) {
      console.error('Unable to copy to clipboard', err)
      alert.error('Copy error')
    }
    document.body.removeChild(textArea)
  }

  if (window.isSecureContext && navigator.clipboard) {
    navigator.clipboard
      .writeText(key)
      .then(() => alert.success(successfulText || 'Copied'))
      .catch(() => alert.error('Copy error'))
  } else {
    unsecuredCopyToClipboard(key)
  }
}
export const isLoggedIn = ({ address }: InjectedAccountWithMeta) =>
  localStorage[LOCAL_STORAGE.ACCOUNT] === address

export const getNotificationTypeValue = (
  str: NotificationResponseTypes,
  tamagotchi?: TamagotchiState
): NotificationType => {
  switch (str) {
    case 'FeedMe':
      return { FeedMe: tamagotchi ? tamagotchi?.fed : undefined }
    case 'PlayWithMe':
      return { PlayWithMe: tamagotchi ? tamagotchi?.entertained : undefined }
    case 'WantToSleep':
      return { WantToSleep: tamagotchi ? tamagotchi?.rested : undefined }
  }
}

export const getStoreItems = (
  state: ItemsStoreResponse,
  programId: HexString
) => {
  if (!state) return { store: [], tamagotchi: [] }
  const store: StoreItemType[] = []
  const tamagotchi: StoreItemsNames[] = []
  for (const idx in state.attributes) {
    const isBought: boolean = state.owners[programId]?.includes(idx)

    if (isBought) tamagotchi.push(state.attributes[+idx][0].media)

    store.push({
      id: idx,
      amount: state.attributes[+idx][1],
      description: state.attributes[+idx][0],
      isBought,
    })
  }
  return { store, tamagotchi }
}
export const getAttributesById = (
  state: ItemsStoreResponse | undefined,
  ids: string[]
): StoreItemsNames[] => {
  if (!state) return []
  if (ids.length < 1) return []

  const result: StoreItemsNames[] = []
  for (const id in state.attributes) {
    if (ids.includes(id)) result.push(state.attributes[+id][0].media)
  }
  return result
}

export const sleep = (s: number) =>
  new Promise((resolve) => setTimeout(resolve, s * 1000))

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
