import { useBattle } from '@/features/battle-tamagotchi/context'
import { useAccount } from '@gear-js/react-hooks'
import { cn } from '@/app/utils'
import { Link } from 'react-router-dom'
import { CreateTamagotchiForm } from '@/features/battle-tamagotchi/components/create-tamagotchi-form'
import { Wallet } from '@/features/wallet'
import { ClockUpdateIcon, WarningIcon } from '@/assets/images'
import { Countdown } from '@/components'

export function RegisterTamagotchi() {
  const { battle } = useBattle()
  const { account } = useAccount()
  return (
    <section className="flex items-center h-[calc(100vh-164px-64px)] container">
      {/*Info*/}
      <div className={cn('basis-[635px]', 'flex flex-col items-center gap-9')}>
        <div className="flex flex-col items-center gap-9 w-full">
          <div className="w-full">
            {account && (
              <div className="grid grid-cols-[auto_auto] items-center w-fit px-8 py-6 gap-x-6 border-2 border-primary-600 rounded-xl font-kanit bg-[#2BD07112]">
                <p className="max-w-[250px] text-lg leading-6 font-bold tracking-[0.72px]">
                  The next Tamagotchi battle{' '}
                  <span className="text-primary-600">
                    will start automatically in:
                  </span>
                </p>
                <div className="grid grid-cols-[auto_auto] items-center gap-x-4">
                  <ClockUpdateIcon className="" />
                  <p className="typo-h2 font-normal tracking-[1.6px]">
                    <Countdown milliseconds={0} />
                  </p>
                </div>
              </div>
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
            <div className="flex items-center py-4 rounded-xl bg-gradient-to-r from-primary-600/[.17] to-transparent">
              <div className="p-5.5">
                <WarningIcon />
              </div>
              <p className="font-medium text-base leading-[22px] font-kanit tracking-[0.64px]">
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
            </div>
          </div>

          {/*<div className="w-full">*/}
          {/*  <Link to={'/test'}>Test page</Link> <Link to={'/battle'}>Battle page</Link>*/}
          {/*</div>*/}
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
