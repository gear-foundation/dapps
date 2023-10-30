import { HexString } from '@polkadot/util/types'
import { ProgramMetadata } from '@gear-js/api'
import { AnyJson } from '@polkadot/types/types'
import { useApi } from '@gear-js/react-hooks'
import { useCallback, useState } from 'react'
import { useProgramMetadata } from '@/app/hooks/api'

export function useOnceReadFullState<T>(
  programId?: HexString,
  meta?: ProgramMetadata,
  payload?: AnyJson
) {
  const { api } = useApi()

  const [state, setState] = useState<T>()
  const [isStateRead, setIsStateRead] = useState(false)
  const [error, setError] = useState('')

  const isPayload = payload !== undefined

  const handleReadState = useCallback(() => {
    console.log('read state:')
    console.log(!!api)
    console.log(!!programId)
    console.log(!!meta)
    console.log(!!isPayload)
    if (!api || !programId || !meta || !isPayload) return
    setIsStateRead(false)
    api.programState
      .read({ programId, payload }, meta)
      .then((res) => res.toHuman() as T)
      .then((state) => setState(state))
      .catch((e) => setError(e))
      .finally(() => setIsStateRead(true))
  }, [meta, api, isPayload, payload, programId])

  return { state, isStateRead, error, handleReadState }
}

export function useOnceReadState<T>({
  programId,
  meta,
  payload,
}: {
  programId?: HexString
  meta: string
  payload?: AnyJson
}) {
  const metadata = useProgramMetadata(meta)
  return useOnceReadFullState<T>(programId, metadata, payload)
}
