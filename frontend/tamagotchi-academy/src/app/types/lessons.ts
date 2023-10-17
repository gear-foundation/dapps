import { HexString } from '@polkadot/util/types'

export type T1 = {
  name: string
  dateOfBirth: string
}

type T2 = {
  entertained: string
  entertainedBlock: string
  fed: string
  fedBlock: string
  owner: string
  rested: string
  restedBlock: string
}

type T3 = {
  allowedAccount: string | null
}

type T4 = {
  energy: string
  power: string
}

type TCustom = {
  isDead: boolean
}

export type TamagotchiState = T1 & T2 & T3 & T4 & TCustom

export type LessonState = {
  step: number
  programId: HexString
}

export type NotificationResponseTypes = 'WantToSleep' | 'PlayWithMe' | 'FeedMe'

export type NotificationType = Partial<
  Record<NotificationResponseTypes, string>
>
