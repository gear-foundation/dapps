import { useAccount } from '@gear-js/react-hooks';
import { Link } from 'react-router-dom';
import { StartDisclaimer, Wallet } from '@dapps-frontend/ui';
import { CreateTamagotchiForm } from 'features/battle/components/create-tamagotchi-form';
import { useBattle } from 'features/battle/context';

export const Home = () => {
  const { battle } = useBattle();
  const { account } = useAccount();
  return (
    <section className="grid grid-rows-[1fr_auto_auto] h-[calc(100vh-216px)]">
      {account && (
        <div className="grow flex flex-col justify-center text-center">
          <img
            className="grow w-full h-30 aspect-[45/56]"
            src="/images/avatar.svg"
            width={448}
            height={560}
            alt="Img"
            loading="lazy"
          />
        </div>
      )}
      <div className="flex flex-col items-center gap-9 m-auto">
        <div className="flex flex-col items-center gap-9 w-full">
          <div className="space-y-6 text-center">
            {account ? (
              battle &&
              (battle.state === 'Registration' ? (
                <h2 className="typo-h2 max-w-[430px] mx-auto">
                  Insert program ID to&nbsp;<span className="text-primary">create a character</span>
                </h2>
              ) : (
                <h2 className="typo-h2 max-w-[430px] mx-auto">
                  Game is on! Go&nbsp;to&nbsp;
                  <Link to="/battle" className="text-primary underline hover:no-underline">
                    battle page
                  </Link>
                </h2>
              ))
            ) : (
              <p className="text-[#D1D1D1]">Connect your account to start the game</p>
            )}
          </div>

          {account ? (
            <div>
              <CreateTamagotchiForm />
              <StartDisclaimer fileName="tamagotchi" wikiPath="Gaming/tamagotchi" className="m-auto mt-9" />
            </div>
          ) : (
            <Wallet />
          )}
        </div>
      </div>
    </section>
  );
};
