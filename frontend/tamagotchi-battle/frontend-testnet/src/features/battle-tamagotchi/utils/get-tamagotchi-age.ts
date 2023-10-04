import { TamagotchiAvatarAge } from '../types/tamagotchi'
import { getMinutes } from '@/app/utils'

export const getTamagotchiAgeDiff = (v: number): TamagotchiAvatarAge => {
  const timeNow = Date.now()
  const diff = +getMinutes(Math.abs(v - timeNow))
  return diff > 60 ? 'old' : diff > 20 ? 'adult' : 'baby'
}
