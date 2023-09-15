import React, { useContext, useEffect, useState } from 'react'
import { XIcon } from 'lucide-react'
import { buttonStyles } from '@gear-js/ui'
import { cn } from '@/app/utils'

import { GameContext } from '@/app/context/ctx-game-score'
import { useMessage } from '@/app/hooks/use-message'

import AvatarIcon from '@/assets/images/game/claim-modal/avatar.png'
import SilverCoinIcon from '@/assets/images/game/silver_coin.svg'
import GoldCoinIcon from '@/assets/images/game/gold_coin.svg'
import TotalCoinsIcon from '@/assets/images/game/claim-modal/total-coins.svg'

import style from './game.module.scss';
import { ChampionsPopup } from '@/components/popups/champions-popup'
import { useGame } from '@/app/context/ctx-game'
import { useAccount } from '@gear-js/react-hooks'

type Props = {
    setOpenModal: React.Dispatch<React.SetStateAction<boolean>>;
};


const GameModal = ({ setOpenModal }: Props) => {
    const { onClaimReward, isPending } = useMessage()
    const { silverCoins, goldCoins } = useContext(GameContext);
    const [allTokens, setAllTokens] = useState(0)
    const silverTokens = silverCoins * 5
    const goldTokens = goldCoins * 10

    const [isShowChampionModal, setShowChampionModal] = useState(false)
    const { allPlayers } = useGame()
    const { account } = useAccount()

    useEffect(() => {
        setAllTokens(silverTokens + goldTokens)
    }, [])

    const onClickClaimReward = () => {
        onClaimReward(silverCoins, goldCoins)
    }

    const onClickShowChampion = () => {
        setShowChampionModal(!isShowChampionModal)
    }

    if (isShowChampionModal) {
        const sortedPlayers = allPlayers
            ? allPlayers.slice().sort((playerA, playerB) => {
                const [_, playerInfoA] = playerA;
                const [__, playerInfoB] = playerB;

                const totalCoinsA = playerInfoA.claimedGoldCoins + playerInfoA.claimedSilverCoins;
                const totalCoinsB = playerInfoB.claimedGoldCoins + playerInfoB.claimedSilverCoins;

                return totalCoinsB - totalCoinsA;
            })
            : [];

        return <ChampionsPopup setIsOpen={setShowChampionModal} isOpen={isShowChampionModal} players={sortedPlayers} />
    }

    return (
        <div className={style.modalContain}>
            <div className={style.modalOverlay}>
                <div className={style.modalContent}>
                    <div className={style.avatar}>
                        <img width={100} src={AvatarIcon} alt="" />
                    </div>
                    <div className={style.close} onClick={() => setOpenModal(false)}>
                        <XIcon />
                    </div>
                    <div className={style.info}>
                        <div className={style.title}>
                            <span className='font-semibold'>Dead mouse,</span>
                            <span className='font-semibold text-[#2BD071]'>Congratulations!</span>
                            <span className='font-extralight'>Your reward</span>

                        </div>
                        <div className={style.total}>
                            <div className={style.coins}>
                                <img src={SilverCoinIcon} width={24} alt="" />
                                <span className='font-semibold'>{silverCoins} x 5 = {silverTokens} </span>
                                <span className='font-extralight'>{account?.balance.unit || 'TVARA'}</span>
                            </div>
                            <div className={style.coins}>
                                <img src={GoldCoinIcon} width={24} alt="" />
                                <span className='font-semibold'>{goldCoins} x 10 = {goldTokens} </span>
                                <span className='font-extralight'>{account?.balance.unit || 'TVARA'}</span>
                            </div>

                        </div>
                        <div className={style.totalTokens}>
                            <img src={TotalCoinsIcon} alt="" />
                            <div className={style.number}>
                                <span className='font-medium text-[40px]'>{allTokens}</span>
                                <span className='font-light italic text-[16px]'>{account?.balance.unit || 'TVARA'}</span>
                            </div>
                        </div>
                        <div className={style.buttons}>
                            <button
                                className={cn(
                                    'btn',
                                    buttonStyles.primary,
                                    isPending && 'btn--loading'
                                )}
                                onClick={onClickClaimReward}
                                disabled={isPending}
                            >
                                <span>Claim reward</span>
                            </button>

                            <button
                                className={cn(
                                    'btn',
                                    buttonStyles.lightGreen
                                )}
                                onClick={onClickShowChampion}
                            >
                                <span>Show champions</span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div >
    )
}

export default GameModal