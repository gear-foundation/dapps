import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { CreateTamagotchiForm } from 'components/forms/create-tamagotchi-form';
import { LoginSection } from 'components/sections/login-section';

export const Home = () => {
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
      <div className={clsx('flex flex-col items-center gap-9', account ? 'mt-12' : 'm-auto')}>
        <div className="flex flex-col items-center gap-9 text-center w-full">
          <div className="space-y-6">
            {account ? (
              <h2 className="typo-h2 max-w-[415px] mx-auto">
                Insert program ID to <span className="text-primary">create a character</span>
              </h2>
            ) : (
              <p className="text-[#D1D1D1]">Connect your account to start the game</p>
            )}
          </div>
          <div className=" w-full">{account ? <CreateTamagotchiForm /> : <LoginSection />}</div>
        </div>
      </div>
    </section>
  );
};
