import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import { TamagotchiAvatarAge } from '@/app/types/tamagotchi'
import { withoutCommas } from '@gear-js/react-hooks'

export const getTamagotchiAge = (v: string) => {
  dayjs.extend(relativeTime)
  return dayjs(+withoutCommas(v)).fromNow(true)
}

export const getTamagotchiAgeDiff = (v: string): TamagotchiAvatarAge => {
  const diff = dayjs().diff(dayjs(+withoutCommas(v)), 'minutes')
  return diff > 60 ? 'old' : diff > 20 ? 'adult' : 'baby'
}
