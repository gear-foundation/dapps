import {
  useHandleCalculateGas,
  useReadFullState,
  useSendMessage,
} from '@gear-js/react-hooks'
import type { HexString } from '@polkadot/util/types'
import { useApp, useFTBalance, useLessons } from '@/app/context'
import type {
  BalanceLogic,
  BalanceMain,
  BalanceStorage,
} from '@/app/types/ft-wallet'
import { AnyJson } from '@polkadot/types/types'

export function useFtMessage() {
  const { metaMain, programId } = useFTBalance()

  const calculateGas = useHandleCalculateGas(programId, metaMain)
  const sendMessage = useSendMessage(programId as HexString, metaMain)

  return (
    payload: AnyJson,
    { onSuccess, onError }: { onSuccess: () => void; onError: () => void }
  ) =>
    calculateGas(payload).then(({ min_limit }) =>
      sendMessage({ payload, gasLimit: min_limit, onError, onSuccess })
    )
}

function useReadFTMain<T>() {
  const { metaMain, programId } = useFTBalance()
  return useReadFullState<T>(programId, metaMain, '0x')
}

function useFTMain() {
  const { state } = useReadFTMain<BalanceMain>()
  return state
}

function useReadFTLogic<T>() {
  const state = useFTMain()
  const { metaLogic } = useFTBalance()
  return useReadFullState<T>(state?.ftLogicId, metaLogic, '0x')
}

function useFTLogic() {
  const { state } = useReadFTLogic<BalanceLogic>()
  return state
}

function useReadFTStorage<T>() {
  const state = useFTLogic()
  const { lesson } = useLessons()
  const { metaStorage } = useFTBalance()
  const getStorageIdByAccount = () => {
    if (state) {
      for (const a of state.idToStorage) {
        if (a[0] === lesson?.programId.charAt(2)) {
          return a[1] as HexString
        }
      }
    }
  }
  return useReadFullState<T>(getStorageIdByAccount(), metaStorage, '0x')
}

export function useFTStorage() {
  const { lesson } = useLessons()
  const { state } = useReadFTStorage<BalanceStorage>()
  const getBalanceByAccountId = () => {
    if (state) {
      for (const a of state.balances) {
        if (a[0] === lesson?.programId) {
          return a[1]
        }
      }
    }
    return '0.00'
  }
  return getBalanceByAccountId()
}

export function useGetFTBalance() {
  const { lesson } = useLessons()
  const sendHandler = useFtMessage()
  const balance = useFTStorage()
  const { setIsPending } = useApp()

  const handler = (cb?: () => void) => {
    const onSuccess = () => {
      setIsPending(false)
      cb && cb()
    }
    const onError = () => {
      setIsPending(false)
      cb && cb()
    }

    setIsPending(true)

    sendHandler(
      {
        Message: {
          transaction_id: Math.floor(Math.random() * Date.now()),
          payload: {
            Mint: {
              amount: 5000,
              recipient: lesson?.programId,
            },
          },
        },
      },
      { onSuccess, onError }
    )
  }

  return { balance, handler }
}
