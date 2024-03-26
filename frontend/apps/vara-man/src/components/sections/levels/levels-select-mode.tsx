import { useApp } from '@/app/context/ctx-app';
import { cn } from '@/app/utils';
import { Button } from '@/components/ui/button';

import { Icons } from '@/components/ui/icons';
import { useNavigate } from 'react-router-dom';

const levels = [
  {
    title: 'Easy',
    enemies: 4,
    speed: 4,
    color: "[--stats-theme:#00FFC4]",
  },
  {
    title: 'Medium',
    enemies: 8,
    speed: 4,
    color: "[--stats-theme:#5984BE]",
  },
  {
    title: 'Hard',
    enemies: 8,
    speed: 8,
    color: "[--stats-theme:#EB5757]",
  },
]

export function LevelsSelectMode() {
  const { isPending } = useApp()
  const navigate = useNavigate()

  return (
    <div className="flex flex-col justify-center items-center grow h-full">
      <h2 className="typo-h2">Difficulty levels</h2>
      <p className="text-[#555756] mt-3">Think carefully and click on any of the difficulty levels.</p>

      <div className="flex gap-7 mt-10 justify-between">
        {levels.map(item => {
          return (
            <div
              key={item.title}
              className={cn(
                "border rounded-2xl text-center cursor-pointer",
                item.color,
                "border-[var(--stats-theme)]"
              )}
              onClick={() => navigate(`/game?level=${item.title}`)}
            >
              <h3 className="text-xl font-semibold p-6">{item.title}</h3>
              <hr className="bg-[var(--stats-theme)] h-[1px] border-none" />
              <div className='p-10 flex flex-col gap-4'>
                <div>
                  {item.enemies} enemies

                  <div className="flex mt-2">
                    {Array.from({ length: 8 }).map((_, index) => {
                      return index < item.enemies ? <Icons.skull key={index} /> : <Icons.skullDisable key={index} />;
                    })}
                  </div>
                </div>
                <div>
                  Low enemy speed
                  <div className="flex mt-2">
                    {Array.from({ length: 8 }).map((_, index) => {
                      return index < item.speed ? <Icons.speedLevel key={index} /> : <Icons.speedLevelDisable key={index} />;
                    })}
                  </div>
                </div>
              </div>
            </div>
          )
        })}
      </div>

      <div className="mt-5">
        <Button variant="gray" className="w-62" onClick={() => navigate("/")}>Back</Button>
      </div>
    </div >
  );
}
