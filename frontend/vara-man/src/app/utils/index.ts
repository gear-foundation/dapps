import { ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'
import { AlertContainerFactory } from '@gear-js/react-hooks'
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types'
import { LOCAL_STORAGE } from '@/app/consts'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function formatDate(input: string | number): string {
  const date = new Date(input)
  return date.toLocaleDateString('en-US', {
    month: 'long',
    day: 'numeric',
    year: 'numeric',
  })
}

export function prettyDate(
  input: number | Date | string,
  options: Intl.DateTimeFormatOptions = {
    dateStyle: 'long',
    timeStyle: 'short',
    hourCycle: 'h23',
  },
  locale: string = 'en-US'
) {
  const date = typeof input === 'string' ? new Date(input) : input
  return new Intl.DateTimeFormat(locale, options).format(date)
}

export function absoluteUrl(path: string) {
  return `${process.env.NEXT_PUBLIC_APP_URL}${path}`
}

export const sleep = (s: number) =>
  new Promise((resolve) => setTimeout(resolve, s * 1000))

export const copyToClipboard = async ({
  alert,
  key,
  successfulText,
}: {
  alert: AlertContainerFactory
  key: string
  successfulText?: string
}) => {
  const onSuccess = () => alert.success(successfulText || 'Copied')
  const onError = () => alert.error('Copy error')

  function unsecuredCopyToClipboard(text: string) {
    const textArea = document.createElement('textarea')
    textArea.value = text
    document.body.appendChild(textArea)
    textArea.focus()
    textArea.select()
    try {
      document.execCommand('copy')
      onSuccess()
    } catch (err) {
      console.error('Unable to copy to clipboard', err)
      onError()
    }
    document.body.removeChild(textArea)
  }

  if (window.isSecureContext && navigator.clipboard) {
    navigator.clipboard
      .writeText(key)
      .then(() => onSuccess())
      .catch(() => onError())
  } else {
    unsecuredCopyToClipboard(key)
  }
}

export const isLoggedIn = ({ address }: InjectedAccountWithMeta) =>
  localStorage[LOCAL_STORAGE.ACCOUNT] === address
