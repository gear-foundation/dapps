import { io } from 'socket.io-client'
import { Client, cacheExchange, fetchExchange } from 'urql'
import { isHex } from '@polkadot/util'
import { HexString } from '@polkadot/util/types'
import { ADDRESS } from 'consts'
import { AlertContainerFactory } from '@gear-js/react-hooks'

export const copyToClipboard = async ({
  alert,
  value,
  successfulText,
}: {
  alert?: AlertContainerFactory
  value: string
  successfulText?: string
}) => {
  const onSuccess = () => {
    if (alert) {
      alert.success(successfulText || 'Copied')
    }
  }
  const onError = () => {
    if (alert) {
      alert.error('Copy error')
    }
  }

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
      .writeText(value)
      .then(() => onSuccess())
      .catch(() => onError())
  } else {
    unsecuredCopyToClipboard(value)
  }
}

export const isProgramIdValid = (value: string): value is HexString =>
  isHex(value, 256)

export const socket = io(ADDRESS.GAME_STATE_SOCKET, {
  transports: ['websocket'],
})

export const urqlClient = new Client({
  url: ADDRESS.EXPLORER_URL || 'https://nft-explorer.vara-network.io/graphql',
  exchanges: [cacheExchange, fetchExchange],
})

export const isMobileDevice =
  /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
    navigator.userAgent
  )
