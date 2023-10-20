import { HexString } from '@polkadot/util/types'

export type StoreItemsNames = 'sword' | 'hat' | 'bag' | 'glasses'

type StoreItemDescription = {
  description: string
  media: StoreItemsNames
  title: string
}

export type StoreItemType = {
  id: string
  amount: string
  description: StoreItemDescription
  isBought: boolean
}

export type ItemsStoreResponse = {
  admin: HexString
  attributes: Record<string, [StoreItemDescription, string]>
  ftContractId: HexString
  owners: Record<HexString, string[]>
  transactionId: string
  transactions: {}
}
