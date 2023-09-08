import { IFTLogic, IFTStorage } from './types'
import { Account } from '@gear-js/react-hooks'

export const getFTStorageIdByAccount = ({
  ids,
  accountAddress,
}: {
  ids?: IFTLogic['idToStorage']
  accountAddress?: Account['decodedAddress']
}) => {
  if (!(accountAddress && ids)) return undefined
  if (ids.length > 0) {
    const address = ids.find(([id]) => id === accountAddress.charAt(2))
    return address ? address[1] : undefined
  }
  return undefined
}

export const getAccountBalanceById = ({
  balances,
  accountAddress,
}: {
  balances?: IFTStorage['balances']
  accountAddress?: Account['decodedAddress']
}) => {
  if (!(accountAddress && balances)) return '0'
  if (balances.length > 0) {
    const balance = balances.find(([id]) => id === accountAddress)
    return balance ? balance[1] : '0'
  }
  return '0'
}
