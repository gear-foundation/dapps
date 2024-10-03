import { useAccount } from '@gear-js/react-hooks';
import { Wallet } from '@dapps-frontend/ui';
import { useLessons, useTamagotchi } from '@/app/context';
import { TamagotchiAvatar } from '@/components/tamagotchi/tamagotchi-avatar';
import { TamagotchiInfoCard } from '@/components/tamagotchi/tamagotchi-info-card';
import { Loader } from '@/components/loaders/loader';
import { CreateTamagotchiForm } from '@/components/forms/create-tamagotchi-form';

export const HomeCreateSection = () => {
  const { account } = useAccount();

  const { tamagotchi } = useTamagotchi();
  const { lesson, isReady } = useLessons();

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
        {lesson ? (
          isReady && <TamagotchiInfoCard />
        ) : (
          <div className="flex flex-col items-center gap-9 text-center w-full">
            <div className="space-y-6">
              {account ? (
                <>
                  <h2 className="typo-h2 text-primary">Geary</h2>
                  <p className="text-[#D1D1D1]">Insert program ID to create a character</p>
                </>
              ) : (
                <h2 className="typo-h2 text-center">
                  <span className="block text-primary">Connect your account</span> to start the game
                </h2>
              )}
            </div>

            {account ? (
              <div className=" w-full">
                <CreateTamagotchiForm />{' '}
              </div>
            ) : (
              <Wallet theme="gear" />
            )}
          </div>
        )}
      </div>
    </section>
  );
};
