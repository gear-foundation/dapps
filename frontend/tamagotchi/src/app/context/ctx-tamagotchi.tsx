import { createContext, ReactNode, useState } from 'react'
import type { TamagotchiState } from '@/app/types/lessons'
import type { StoreItemsNames } from '@/app/types/ft-store'

export const TamagotchiCtx = createContext({} as ReturnType<typeof useProgram>)

const useProgram = () => {
  const [tamagotchi, setTamagotchi] = useState<TamagotchiState>()
  const [tamagotchiItems, setTamagotchiItems] = useState<StoreItemsNames[]>([])

  return {
    tamagotchi,
    setTamagotchi,
    tamagotchiItems,
    setTamagotchiItems,
  }
}

export function TmgProvider({ children }: { children: ReactNode }) {
  const { Provider } = TamagotchiCtx
  return <Provider value={useProgram()}>{children}</Provider>
}
