import { Icons } from '@/components/ui/icons'
import { LevelsBackground } from '@/components/sections/levels/levels-background'
import { LevelsModeContent } from '@/components/sections/levels/levels-mode-content'
import { LevelsStartAction } from '@/components/sections/levels/levels-start-action'

import LevelsBackgroundImage from '@/assets/images/levels/bg2.jpg'
import { useGame } from '@/app/context/ctx-game'

export function LevelsMedium() {
  const { player } = useGame()

  const lifeIcons = Array.from({ length: 3 }, (_, index) => (
    <div key={index}>
      <Icons.lifes
        primary={Number(player?.lives) > index ? "currentColor" : "gray"}
        secondary={Number(player?.lives) > index ? "1E8C4D" : "gray"}
        className="w-9 h-9"
      />
    </div>
  ));

  return (
    <>
      <div className="relative grow">
        <LevelsBackground picture={LevelsBackgroundImage} />
      </div>
      {/*Level info*/}
      <LevelsModeContent title="Medium" colorText="text-[#F46402]">
        <div className="mt-8">
          <ul>
            <li>
              <div className="flex items-center py-2.5 pl-6 space-x-7">
                <span className="text-base w-25">Enemies:</span>
                <div className="grid grid-cols-3 gap-4 text-[#F46402]">
                  <div>
                    <Icons.deathActive className="w-9 h-9" />
                  </div>
                  <div>
                    <Icons.deathActive className="w-9 h-9" />
                  </div>
                  <div>
                    <Icons.deathInactive className="w-9 h-9" />
                  </div>
                </div>
              </div>
            </li>
            <li>
              <div className="flex items-center py-2.5 pl-12 space-x-7">
                <span className="text-base w-25">Speed:</span>
                <div className="grid grid-cols-3 gap-4 text-[#F46402]">
                  <div>
                    <Icons.flameActive
                      secondary="#933F0D"
                      className="w-9 h-9"
                    />
                  </div>
                  <div>
                    <Icons.flameActive
                      secondary="#933F0D"
                      className="w-9 h-9"
                    />
                  </div>
                  <div>
                    <Icons.flameInactive
                      secondary="#919191"
                      className="w-9 h-9"
                    />
                  </div>
                </div>
              </div>
            </li>
            <li>
              <div className="flex items-center py-2.5 pl-18 space-x-7">
                <span className="text-base w-25">Rewards:</span>
                <div className="grid grid-cols-3 gap-4 text-[#F46402]">
                  <div>
                    <Icons.coins1 secondary="#933F0D" className="w-9 h-9" />
                  </div>
                  <div>
                    <Icons.coins2 secondary="#933F0D" className="w-9 h-9" />
                  </div>
                  <div>
                    <Icons.coins3
                      secondary="#919191"
                      className="w-9 h-9 text-[#626262]"
                    />
                  </div>
                </div>
              </div>
            </li>
            <li>
              <div className="flex items-center py-2.5 pl-24 space-x-7">
                <span className="text-base w-25">Lives left:</span>
                <div className="grid grid-cols-3 gap-4 text-[#F46402]">
                  {lifeIcons}
                </div>
              </div>
            </li>
          </ul>
        </div>
        <LevelsStartAction
          className="bg-[#F46402] hover:bg-[#933F0D]"
          level="Medium"
        />
      </LevelsModeContent>
    </>
  )
}
