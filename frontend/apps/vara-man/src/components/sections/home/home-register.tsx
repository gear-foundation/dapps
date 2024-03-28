import { ArrowRight, Search } from 'lucide-react'
import { WalletNew } from '@dapps-frontend/ui/'
import { useAccount } from '@gear-js/react-hooks';
import IntroImage from '@/assets/images/welcome.png'
import { Icons } from '@/components/ui/icons';
import { useNavigate, NavigateFunction } from 'react-router-dom';

const selectMode = [
  {
    title: "Just play and have fun!",
    description: "Start the game without any preparations right now.",
    icon: <Icons.gameJoystick />,
    onClick: (navigate: NavigateFunction): void => {
      navigate('/levels')
    }
  },
  {
    title: "Find a private game",
    description: "To find the game, you need to enter the administrator's address.",
    icon: <Icons.search />,
    onClick: (navigate: NavigateFunction): void => {
      navigate('/tournament/find')
    }
  },
  {
    title: "Create a game in administrator mode",
    description: "Create a game and specify your participation rules.",
    icon: <Icons.admin />,
    onClick: (navigate: NavigateFunction): void => {
      navigate('/tournament/create')
    }
  }
]

export function HomeRegister() {
  const { account } = useAccount();
  const navigate = useNavigate();

  return (
    <>
      <div className="flex justify-between items-center grow h-full">
        {account ?
          <div>
            <h2 className='typo-h2'>Select game mode</h2>
            <p className="text-[#555756] mt-3">Which mode shall we play in today?</p>

            <div className="flex flex-col gap-3 mt-10">
              {selectMode.map(item => {
                return (
                  <button
                    key={item.title}
                    className="flex justify-between items-center p-4 border rounded-2xl gap-5 hover:border-[#00FFC4] disabled:opacity-50 hover:disabled:border-[#f2f2f2]"
                    onClick={() => item.onClick(navigate)}
                  >
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
                )
              })}
            </div>
          </div> :
          <div className="relative w-full max-w-[550px]">
            <h2 className='typo-h2'>Welcome, treasure hunter!</h2>
            <p className="text-[#555756] mt-3">In this game, you can test your strength in the quest for treasures.
              The game offers various difficulty levels and game modes. Connect your wallet.
            </p>

            <div className='mt-3'>
              <WalletNew />
            </div>
          </div>
        }
        <div>
          <img src={IntroImage} alt="" className="rounded-3xl" />
        </div>
      </div>
    </>
  );
}
