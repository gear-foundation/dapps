import { HexString } from '@polkadot/util/types'

export type IFTMain = {
  admin: HexString
  ftLogicId: HexString
  transactions: []
}

export type IFTLogic = {
  admin: HexString
  ftokenId: HexString
  idToStorage: Array<[string, HexString]>
  instructions: []
  storageCodeHash: HexString
  transactionStatus: [HexString, 'Failure' | 'Success'][]
}

export type IFTStorage = {
  approvals: []
  balances: Array<[HexString, string]>
  ftLogicId: HexString
  transactionStatus: []
}
