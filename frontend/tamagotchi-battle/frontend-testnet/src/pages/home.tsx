import { BattleTamagotchi } from '@/features/battle-tamagotchi/battle-tamagotchi'
import { RegisterTamagotchi } from '@/features/battle-tamagotchi/register-tamagotchi'
import { useBattle } from '@/features/battle-tamagotchi/context'
import { useAccount } from '@gear-js/react-hooks'
import { BattlePlayersQueue } from '@/features/battle-tamagotchi/components/battle-players-queue'
import { useMemo } from 'react'
import { NextBattleTimer } from '@/features/battle-tamagotchi/components/next-battle-timer'
import { InfoCard } from '@/features/battle-tamagotchi/components/info-card'

export default function Home() {
  const { account } = useAccount()
  const { battle } = useBattle()
  // return <BattleTamagotchi />
  const isRegistered = useMemo(
    () =>
      battle?.heroes
        ? Object.values(battle?.heroes).find(
            (hero) => hero.owner === account?.decodedAddress
          )
        : false,
    [account?.decodedAddress, battle?.heroes]
  )

  if (!battle || !account) return null

  return (
    <>
      {!isRegistered && ['Unknown', 'Registration'].includes(battle.status) && (
        <RegisterTamagotchi battle={battle} />
      )}
      {isRegistered && Object.keys(battle.heroes).length > 0 && (
        <>
          <div className="flex flex-col grow max-w-[560px] max-h-[440px] my-auto mx-auto pb-10">
            <div className="space-y-6 text-center pb-6">
              <NextBattleTimer
                battle={battle}
                className="pl-6 pr-8 rounded-full mx-auto"
              />
              <h2 className="typo-h2">Wait for the game to start</h2>
              <p className="text-white/80 max-w-[370px] mx-auto">
                Wait for the end of registration of other participants. As soon
                as the timer expires, the game will begin.
              </p>
            </div>
            <InfoCard className="mt-auto">
              <p>
                All game mechanics and automatic launch are facilitated by
                on-chain mechanisms and delayed messages. You can read more
                about these capabilities of Vara Network{' '}
                <a
                  href="#"
                  target="_blank"
                  className="text-primary-600 underline hover:no-underline hover:text-primary"
                >
                  in our Wiki
                </a>
                .
              </p>
            </InfoCard>
          </div>
          <BattlePlayersQueue />
        </>
      )}
    </>
  )
}
