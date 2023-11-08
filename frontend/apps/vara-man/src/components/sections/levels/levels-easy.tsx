import { Icons } from '@/components/ui/icons'
import { buttonStyles } from '@gear-js/ui'
import { LevelsBackground } from '@/components/sections/levels/levels-background'
import { LevelsModeContent } from '@/components/sections/levels/levels-mode-content'
import { LevelsStartAction } from '@/components/sections/levels/levels-start-action'

import LevelsBackgroundImage from '@/assets/images/levels/bg1.jpg'
import { useGame } from '@/app/context/ctx-game'

export function LevelsEasy() {
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
      <LevelsModeContent title="Easy" colorText="text-primary">
        <div className="mt-8">
          <ul>
            <li>
              <div className="flex items-center py-2.5 pl-6 space-x-7">
                <span className="text-base w-25">Enemies:</span>
                <div className="grid grid-cols-3 gap-4 text-primary">
                  <div>
                    <Icons.deathActive className="w-9 h-9" />
                  </div>
                  <div>
                    <Icons.deathInactive className="w-9 h-9" />
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
                <div className="grid grid-cols-3 gap-4 text-primary">
                  <div>
                    <Icons.flameActive
                      secondary="#1E8C4D"
                      className="w-9 h-9"
                    />
                  </div>
                  <div>
                    <Icons.flameInactive
                      secondary="#919191"
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
                <div className="grid grid-cols-3 gap-4 text-primary">
                  <div>
                    <Icons.coins1 secondary="#1E8C4D" className="w-9 h-9" />
                  </div>
                  <div>
                    <Icons.coins2
                      secondary="#919191"
                      className="w-9 h-9 text-[#626262]"
                    />
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
                <div className="grid grid-cols-3 gap-4 text-primary">
                  {lifeIcons}
                </div>
              </div>
            </li>
          </ul>
          <LevelsStartAction className={buttonStyles.primary} level="Easy" />
        </div>
      </LevelsModeContent>
    </>
  )
}
