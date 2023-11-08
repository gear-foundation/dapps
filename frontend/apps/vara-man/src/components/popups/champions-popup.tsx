import {
  PopupContainer,
  PopupContainerProps,
} from '@/components/common/popup-container'
import { ScrollArea } from '@/components/ui/scroll-area'
import { cn } from '@/app/utils'
import { buttonStyles } from '@gear-js/ui'
import { XIcon } from 'lucide-react'
import { IPlayer } from '@/app/types/game'

import GoldCoinIcon from '@/assets/images/game/gold_coin.svg'
import SilverCoinIcon from '@/assets/images/game/silver_coin.svg'
import AvatarIcon from '@/assets/images/champions/avarar.svg'

import styles from './champions-popup.module.scss'

type ChampionsPopupProps = PopupContainerProps & {
  players: IPlayer[]
}

export function ChampionsPopup({
  isOpen,
  setIsOpen,
  players,
  overlayCn,
  className,
}: ChampionsPopupProps) {
  const onClose = () => setIsOpen((_) => !_)

  return (
    <PopupContainer
      {...{
        isOpen,
        setIsOpen,
        overlayCn,
        className,
      }}
      title="Champions"
      footer={
        <div className="pr-4 pl-8 pt-5 pb-8">
          <button
            className={cn(
              'btn font-kanit w-full space-x-2',
              buttonStyles.light
            )}
            onClick={onClose}
          >
            <XIcon className="w-5 h-5 text-white/80" />
            <span className="leading-4">Close</span>
          </button>
        </div>
      }
    >
      <div className="font-kanit">
        <div className="flex justify-between px-4 leading-6 text-xs tracking-[0.08em] uppercase text-white/60 bg-white/5 rounded-[20px]">
          <span>Player</span>
          <span>Coins</span>
        </div>
      </div>
      <ScrollArea className="mt-3 max-h-80 pr-4 -mr-4" type="auto">
        {players.map((item, index) => {
          const { name, claimedGoldCoins, claimedSilverCoins } = item[1]

          const playerClassNames = [
            styles.player,
            index === 0 && styles.first,
            index === 1 && styles.second,
            index === 2 && styles.thirty,
          ];

          return (
            <div
              className={cn(playerClassNames)}
              key={item[0]}
            >
              <img src={AvatarIcon} alt="" className="-m-[10px]" />
              <span className="w-50 ml-5">{name}</span>

              <div className="flex items-center justify-end">
                {[
                  { icon: GoldCoinIcon, value: claimedGoldCoins },
                  { icon: SilverCoinIcon, value: claimedSilverCoins }
                ].map((coin, coinIndex) => (
                  <div className="flex items-center w-12" key={coinIndex}>
                    <img width={20} src={coin.icon} alt="" className="mr-1" />
                    <span>{coin.value}</span>
                  </div>
                ))}
              </div>
            </div>
          );
        })}
      </ScrollArea >
    </PopupContainer >
  )
}
