import { useAccount } from '@gear-js/react-hooks'
import { cn } from '@/app/utils'
import { CreateTamagotchiForm } from '@/features/battle-tamagotchi/components/create-tamagotchi-form'
import { Wallet } from '@/features/wallet'
import { BattleStateResponse } from '@/features/battle-tamagotchi/types/battles'
import { NextBattleTimer } from '@/features/battle-tamagotchi/components/next-battle-timer'
import { InfoCard } from '@/features/battle-tamagotchi/components/info-card'

export function RegisterTamagotchi({
  battle,
}: {
  battle: BattleStateResponse
}) {
  const { account } = useAccount()

  return (
    <section className="flex items-center h-[calc(100vh-164px-64px)] container">
      {/*Info*/}
      <div className={cn('basis-[635px]', 'flex flex-col items-center gap-9')}>
        <div className="flex flex-col items-center gap-9 w-full">
          <div className="w-full">
            {account && (
              <NextBattleTimer
                battle={battle}
                className="grid-cols-[auto_auto]"
              >
                <p className="max-w-[250px] text-lg leading-6 font-bold tracking-[0.72px]">
                  The next Tamagotchi battle{' '}
                  <span className="text-primary-600">
                    will start automatically in:
                  </span>
                </p>
              </NextBattleTimer>
            )}
          </div>

          <div className="w-full space-y-6">
            {account ? (
              battle && (
                <>
                  {battle.status === 'Unknown' && (
                    <h2 className="typo-h2">The Game is paused</h2>
                  )}
                  {battle.status === 'Registration' && (
                    <h2 className="typo-h2">
                      Insert your Tamagotchi program ID to{' '}
                      <span className="text-primary">Register in battle</span>
                    </h2>
                  )}
                </>
              )
            ) : (
              <p className="typo-h2 text-[#D1D1D1]">
                Connect your account <br />
                to <span className="text-primary">start the game</span>
              </p>
            )}
          </div>

          <div className="w-full">
            {account ? <CreateTamagotchiForm /> : <Wallet account={account} />}
          </div>

          <div className="">
            <InfoCard>
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
        </div>
      </div>
      {/*Image*/}
      <div className="relative bottom-[10%] -z-1 flex flex-col justify-center basis-[450px] self-stretch">
        <img
          className="grow w-full h-30 aspect-[45/56]"
          src="/images/avatar.svg"
          width={448}
          height={560}
          alt="Img"
          loading="lazy"
        />
      </div>
    </section>
  )
}
