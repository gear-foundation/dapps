import { useEffect } from 'react';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useNavigate, NavigateFunction } from 'react-router-dom';
import { ArrowRight, Search } from 'lucide-react';
import { WalletNew } from '@dapps-frontend/ui/';
import IntroImage from '@/assets/images/welcome.png';
import { Icons } from '@/components/ui/icons';
import { EzTransactionsSwitch, useEzTransactions } from '@dapps-frontend/ez-transactions';
import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';

const selectMode = [
  {
    title: 'Just play and have fun!',
    description: 'Start the game without any preparations right now.',
    icon: <Icons.gameJoystick />,
    onClick: (navigate: NavigateFunction): void => {
      navigate('/levels');
    },
  },
  {
    title: 'Find a private game',
    description: "To find the game, you need to enter the administrator's address.",
    icon: <Icons.search />,
    onClick: (navigate: NavigateFunction): void => {
      navigate('/tournament/find');
    },
  },
  {
    title: 'Create your private game',
    description: 'Create your own game tournament and compete with friends.',
    icon: <Icons.admin />,
    onClick: (navigate: NavigateFunction): void => {
      navigate('/tournament/create');
    },
  },
];

export function HomeRegister() {
  const { account } = useAccount();
  const navigate = useNavigate();
  const { gasless, signless } = useEzTransactions();
  const alert = useAlert();

  const startGame = (onClick: void) => {
    if (account) {
      if (!gasless.isEnabled || gasless.voucherId || signless.isActive) return onClick;

      gasless.requestVoucher(account.address).catch(({ message }: Error) => alert.error(message));
    }
  };

  return (
    <>
      <div className="flex flex-col md:flex-row justify-between items-center md:grow h-full">
        {account ? (
          <div className="md:text-left text-center flex flex-col md:flex-none">
            <h2 className="typo-h2">Select game mode</h2>
            <p className="text-[#555756] mt-3">Which mode shall we play in today?</p>

            <div className="flex flex-col gap-3 mt-5">
              {selectMode.map((item) => {
                return (
                  <button
                    key={item.title}
                    className="flex justify-between md:text-center text-left items-center p-4 md:border md:border-[#e5e7eb] border border-[#00FFC4] rounded-2xl gap-5 hover:border-[#00FFC4] disabled:opacity-50 hover:disabled:border-[#f2f2f2]"
                    onClick={() => startGame(item.onClick(navigate))}
                    disabled={gasless.isLoading}>
                    <div className="flex flex-col items-start">
                      <div className="flex gap-2 mb-2">
                        {item.icon}
                        <h3 className="font-semibold">{item.title}</h3>
                      </div>
                      <span className="text-[#4D4D4D]">{item.description}</span>
                    </div>
                    <div>
                      <ArrowRight />
                    </div>
                  </button>
                );
              })}
            </div>
            <div className="mt-5">
              <EzTransactionsSwitch allowedActions={SIGNLESS_ALLOWED_ACTIONS} />
            </div>
          </div>
        ) : (
          <div className="relative w-full max-w-[550px] md:text-left text-center flex flex-col items-center md:items-start md:flex-none">
            <h2 className="typo-h2">Welcome, treasure hunter!</h2>
            <p className="text-[#555756] mt-3">
              In this game, you can test your strength in the quest for treasures. The game offers various difficulty
              levels and game modes. Connect your wallet.
            </p>

            <div className="mt-3">
              <WalletNew />
            </div>
          </div>
        )}
        <div className="order-first md:-order-none relative md:h-full h-63 w-full md:w-max">
          <img src={IntroImage} alt="" className="rounded-3xl h-full object-cover w-full" />
          <div className="absolute -inset-1 md:bg-none bg-gradient-to-t from-white to-transparent rounded-3xl"></div>
        </div>
      </div>
    </>
  );
}
