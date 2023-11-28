import dayjs from 'dayjs'
import { TamagotchiAvatarAge } from '../types/tamagotchi'

export const getTamagotchiAgeDiff = (v: number): TamagotchiAvatarAge => {
  const diff = dayjs().diff(dayjs(v), 'minutes')
  return diff > 60 ? 'old' : diff > 20 ? 'adult' : 'baby'
}
