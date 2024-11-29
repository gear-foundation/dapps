import { cn } from '@/app/utils';
import { Button } from '@/components/ui/button';

import { Icons } from '@/components/ui/icons';
import { useNavigate } from 'react-router-dom';

const levels = [
  {
    id: 'Easy',
    title: 'Easy!',
    enemies: 4,
    speed: 4,
    descriptionSpeed: 'Low enemy speed',
    color: '[--stats-theme:#00FFC4]',
  },
  {
    id: 'Medium',
    title: 'Medium',
    enemies: 8,
    speed: 4,
    descriptionSpeed: 'Low enemy speed',
    color: '[--stats-theme:#5984BE]',
  },
  {
    id: 'Hard',
    title: 'Hardcore',
    enemies: 8,
    speed: 8,
    descriptionSpeed: 'High enemy speed',
    color: '[--stats-theme:#EB5757]',
  },
];

export function LevelsSelectMode() {
  const navigate = useNavigate();

  const startSingleGame = (level: string) => {
    navigate(`/game?level=${level}`);
  };

  return (
    <div className="flex flex-col justify-center items-center grow h-full text-center md:text-left">
      <h2 className="typo-h2">Difficulty levels</h2>
      <p className="text-[#555756] mt-3">Think carefully and click on any of the difficulty levels.</p>

      <div className="flex md:flex-row flex-col gap-7 mt-10 justify-between w-full md:w-max">
        {levels.map((item) => {
          return (
            <div
              key={item.title}
              className={cn('border rounded-2xl text-center cursor-pointer', item.color, 'border-[var(--stats-theme)]')}
              onClick={() => startSingleGame(item.id)}>
              <h3 className="text-xl font-semibold md:p-6 p-3 md:text-center text-left">{item.title}</h3>
              <hr className="bg-[var(--stats-theme)] h-[1px] border-none" />
              <div className="md:p-10 p-3 flex md:flex-col flex-row justify-between gap-4 md:text-center text-left">
                <div>
                  {item.enemies} enemies
                  <div className="flex mt-2">
                    {Array.from({ length: 8 }).map((_, index) => {
                      return index < item.enemies ? <Icons.skull key={index} /> : <Icons.skullDisable key={index} />;
                    })}
                  </div>
                </div>
                <div>
                  {item.descriptionSpeed}
                  <div className="flex mt-2">
                    {Array.from({ length: 8 }).map((_, index) => {
                      return index < item.speed ? (
                        <Icons.speedLevel key={index} />
                      ) : (
                        <Icons.speedLevelDisable key={index} />
                      );
                    })}
                  </div>
                </div>
              </div>
              {item.title === 'Hardcore' && (
                <div className="bg-[#EB5757] rounded-b-2xl border border-[var(--stats-theme)] font-semibold text-white flex justify-center items-center gap-1.5">
                  <Icons.blindMode /> Blind mode
                </div>
              )}
            </div>
          );
        })}
      </div>

      <div className="mt-5">
        <Button variant="gray" className="w-62" onClick={() => navigate('/')}>
          Back
        </Button>
      </div>
    </div>
  );
}
