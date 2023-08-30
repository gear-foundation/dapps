import { useAlert, useReadFullState } from '@gear-js/react-hooks'
import {
  getProgramMetadata,
  getStateMetadata,
  ProgramMetadata,
  StateMetadata,
} from '@gear-js/api'
import { HexString } from '@polkadot/util/types'
import { useEffect, useState } from 'react'

export function useProgramMetadata(source: string) {
  const alert = useAlert()

  const [metadata, setMetadata] = useState<ProgramMetadata>()

  useEffect(() => {
    fetch(source)
      .then((response) => response.text())
      .then((raw) => `0x${raw}` as HexString)
      .then((metaHex) => getProgramMetadata(metaHex))
      .then((result) => setMetadata(result))
      .catch(({ message }: Error) => alert.error(message))

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  return metadata
}

export function useStateMetadata(source: string) {
  const alert = useAlert()

  const [stateMetadata, setStateMetadata] = useState<StateMetadata>()

  useEffect(() => {
    fetch(source)
      .then((response) => response.arrayBuffer())
      .then((arrayBuffer) => Buffer.from(arrayBuffer))
      .then((buffer) => getStateMetadata(buffer))
      .then((result) => setStateMetadata(result))
      .catch(({ message }: Error) => alert.error(message))

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  return stateMetadata
}

export function useReadState<T>({
  programId,
  meta,
}: {
  programId?: HexString
  meta: string
}) {
  const metadata = useProgramMetadata(meta)
  return useReadFullState<T>(programId, metadata)
}
