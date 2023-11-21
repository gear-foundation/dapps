import meta from './assets/meta/ft_main.meta.txt'
import metaFTLogic from './assets/meta/ft_logic.meta.txt'
import metaFTStorage from './assets/meta/ft_storage.meta.txt'
import { useEffect, useState } from 'react'
import { useAtomValue, useSetAtom } from 'jotai'
import { useAccount } from '@gear-js/react-hooks'
import { useReadState } from '@/app/hooks/api'
import { FT_BALANCE, FT_BALANCE_READY } from './store'
import { IFTLogic, IFTMain, IFTStorage } from './types'
import { getAccountBalanceById, getFTStorageIdByAccount } from './utils'
import { ADDRESS } from './consts'
import { HexString } from '@polkadot/util/types'

export function useFTBalance() {
  const setBalance = useSetAtom(FT_BALANCE)
  const setFTBalanceReady = useSetAtom(FT_BALANCE_READY)
  const balance = useAtomValue(FT_BALANCE)
  const isFTBalanceReady = useAtomValue(FT_BALANCE_READY)
  return {
    balance,
    setBalance,
    isFTBalanceReady,
    setFTBalanceReady,
  }
}

function useFTStorage() {
  const { account } = useAccount()
  const { state: stateMain, error: errorMain } = useReadState<IFTMain>({
    programId: ADDRESS.SFT,
    meta,
  })
  const { state: stateLogic, error: errorLogic } = useReadState<IFTLogic>({
    programId: stateMain?.ftLogicId,
    meta: metaFTLogic,
  })
  const [storageId, setStorageId] = useState<HexString | undefined | null>(null)
  const [isIdExist, setIsIdExist] = useState<boolean | null>(null)

  useEffect(() => {
    if (stateLogic) {
      setStorageId(
        getFTStorageIdByAccount({
          ids: stateLogic?.idToStorage,
          accountAddress: account?.decodedAddress,
        })
      )
    }
  }, [account, stateLogic])

  const { state, error } = useReadState<IFTStorage>({
    programId: storageId !== null ? storageId : undefined,
    meta: metaFTStorage,
  })

  useEffect(() => {
    if (storageId !== null && stateLogic) {
      setIsIdExist(!!storageId)
    }
  }, [storageId, stateLogic, account])

  return { state, error: error || errorLogic || errorMain, isIdExist }
}

export function useFTBalanceSync() {
  const { account } = useAccount()
  const { setBalance, setFTBalanceReady, isFTBalanceReady } = useFTBalance()
  const { state: stateStorage, isIdExist, error } = useFTStorage()

  useEffect(() => {
    if (isIdExist !== null) {
      setBalance(
        getAccountBalanceById({
          accountAddress: account?.decodedAddress,
          balances: stateStorage?.balances,
        })
      )

      const getStorageReadState = () => {
        if (isIdExist !== null) {
          return isIdExist ? isIdExist && !!stateStorage?.balances : true
        }
        return false
      }

      if (!isFTBalanceReady && getStorageReadState()) {
        setFTBalanceReady(true)
      }
    }
  }, [account, isFTBalanceReady, isIdExist, stateStorage?.balances])

  return {
    errorFT: error,
  }
}
