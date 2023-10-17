import { useLessons, useTamagotchi } from '@/app/context'
import { TamagotchiAvatar } from '@/components/tamagotchi/tamagotchi-avatar'
import { TamagotchiInfoCard } from '@/components/tamagotchi/tamagotchi-info-card'
import { ConnectAccount } from '@/components/common/connect-account'
import { Loader } from '@/components/loaders/loader'

export const HomeCreateSection = () => {
  const { tamagotchi } = useTamagotchi()
  const { lesson, isReady } = useLessons()
  return (
    <section className="grid grid-rows-[1fr_auto_auto] h-[calc(100vh-216px)]">
      <div className="grow flex flex-col justify-center text-center">
        {lesson ? (
          tamagotchi && (isReady ? <TamagotchiAvatar /> : <Loader />)
        ) : (
          <img
            className="grow w-full h-30 aspect-[45/56]"
            src="/images/avatar.svg"
            width={448}
            height={560}
            alt="Img"
            loading="lazy"
          />
        )}
      </div>
      <div className="mt-12 flex flex-col items-center gap-9">
        {lesson ? isReady && <TamagotchiInfoCard /> : <ConnectAccount />}
      </div>
    </section>
  )
}
