import { useEffect, useState } from 'react'
import {
  useAccount,
  useReadFullState,
  useSendMessageHandler,
  withoutCommas,
} from '@gear-js/react-hooks'
import { useLessons, useTamagotchi } from '@/app/context'
import type { HexString } from '@polkadot/util/types'
import type { TamagotchiState } from '@/app/types/lessons'
import { AnyJson } from '@polkadot/types/types'

export function useReadTamagotchi<T>() {
  const { lesson, lessonMeta } = useLessons()
  return useReadFullState<T>(lesson?.programId, lessonMeta, '0x')
}

export function useTamagotchiInit() {
  const { account } = useAccount()
  const { setTamagotchi, tamagotchi } = useTamagotchi()
  const { setIsAdmin, resetLesson, lesson, setIsReady } = useLessons()
  const { state, isStateRead, error } = useReadTamagotchi<TamagotchiState>()
  const [isInit, setIsInit] = useState<boolean>(false)

  useEffect(() => {
    if (!tamagotchi && isInit) {
      setIsInit(false)
    }
  }, [tamagotchi, isInit])

  useEffect(() => {
    if (error) {
      setTamagotchi(undefined)
      resetLesson()
    }
    if (state && isStateRead && account && !isInit) {
      const { fed, rested, entertained, owner, allowedAccount } = state

      if (lesson && lesson.step < 2) {
        setIsReady(true)
      }

      setTamagotchi({
        ...state,
        isDead:
          [
            +withoutCommas(fed),
            +withoutCommas(rested),
            +withoutCommas(entertained),
          ].reduce((sum, a) => sum + a) === 0,
      })

      const { decodedAddress } = account
      setIsAdmin([owner, allowedAccount].includes(decodedAddress))
      setIsInit((prev) => !prev)
    }
  }, [state, isStateRead, account, error, isInit, lesson])
}

export function useTamagotchiMessage() {
  const { lesson, lessonMeta } = useLessons()

  const sendMessage = useSendMessageHandler(
    lesson?.programId as HexString,
    lessonMeta
  )

  return (
    payload: AnyJson,
    { onSuccess, onError }: { onSuccess?: () => void; onError?: () => void }
  ) => sendMessage({ payload, onError, onSuccess })
}
