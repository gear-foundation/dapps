import { useEffect } from 'react'
import { useReadState } from './use-metadata'
import metaStore from '@/assets/meta/meta-store.txt'
import { ENV } from '@/app/consts'
import { useFTStore, useLessons, useTamagotchi } from '@/app/context'
import { ItemsStoreResponse } from '@/app/types/ft-store'
import { getStoreItems } from '@/app/utils'

export function useItemsStore() {
  const { setTamagotchiItems } = useTamagotchi()
  const { lesson } = useLessons()
  const { setItems, setStore } = useFTStore()
  const state = useReadState<ItemsStoreResponse>({
    programId: ENV.store,
    meta: metaStore,
  }).state

  useEffect(() => {
    setStore(state)

    return () => {
      setStore(undefined)
    }
  }, [state])

  useEffect(() => {
    if (lesson && lesson.step > 3 && state) {
      const { programId } = lesson
      setItems(getStoreItems(state, programId).store)
      setTamagotchiItems(getStoreItems(state, programId).tamagotchi)
    } else {
      setItems([])
      setTamagotchiItems([])
    }
    return () => {
      setItems([])
      setTamagotchiItems([])
    }
  }, [lesson, state])
}
