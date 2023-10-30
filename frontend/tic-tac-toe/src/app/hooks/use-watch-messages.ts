import { useAccount, useAlert, useApi } from '@gear-js/react-hooks'
import { useProgramMetadata } from '@/app/hooks/api'
import { MutableRefObject, useRef, useState } from 'react'
import { UnsubscribePromise } from '@polkadot/api/types'
import { Bytes } from '@polkadot/types'
import { UserMessageSent } from '@gear-js/api'
import { ADDRESS } from '@/features/tic-tac-toe/consts'
import metaTxt from '@/features/tic-tac-toe/assets/meta/tic_tac_toe.meta.txt'
import { ContractError } from '../types'

const programId = ADDRESS.GAME

export function useWatchMessages<T>() {
  const { api } = useApi()
  const { account } = useAccount()
  const alert = useAlert()

  const meta = useProgramMetadata(metaTxt)

  const messageSub: MutableRefObject<UnsubscribePromise | null> = useRef(null)
  const [reply, setReply] = useState<T | undefined>()
  const [isOpened, setIsOpened] = useState<boolean>(false)

  const getDecodedPayload = <T>(payload: Bytes) => {
    if (!meta?.types.handle.output) return
    return meta.createType(meta.types.handle.output, payload).toHuman() as T
  }

  const onChangeState = ({ data: { message } }: UserMessageSent) => {
    console.log('onChangeState message:')
    console.log(message.toHuman())
    const { destination, source, payload } = message

    const isOwner = destination.toHex() === account?.decodedAddress
    const isGame = source.toHex() === programId

    if (isOwner && isGame) {
      try {
        const reply = getDecodedPayload<T>(payload)
        console.log('inside update: ', { reply })

        setReply(reply)
      } catch (e) {
        console.log(e)
        alert.error((e as ContractError).message)
      }
    }
  }

  const subscribe = () => {
    if (!api || messageSub.current) return
    console.log('subscribed!')

    setIsOpened(true)
    messageSub.current = api.gearEvents.subscribeToGearEvent(
      'UserMessageSent',
      onChangeState
    )
  }

  const unsubscribe = () => {
    console.log('unsubscribed  :)')
    messageSub.current?.then((unsubCb) => {
      messageSub.current = null
      unsubCb()
      setIsOpened(false)
      setReply(undefined)
    })
  }

  return {
    subscribe,
    unsubscribe,
    reply,
    isOpened,
  }
}
