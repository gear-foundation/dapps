export const toSeconds = (n: number) => {
  const N = Math.abs(n)
  return N < 10 ? `0${N}` : `${N}`
}

export { getTamagotchiAgeDiff } from './get-tamagotchi-age'
export { getTamagotchiColor } from './get-tamagotchi-color'
