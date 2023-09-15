import React, { createContext, useState, useEffect } from 'react';
import { useGame } from './ctx-game';
import { retriesToLivesMap } from '../consts';
import { IGameLevel } from '../types/game';

const timer = 60 * 10

export interface GameContextProps {
    silverCoins: number;
    goldCoins: number;
    incrementCoins: (coinType: 'silver' | 'gold') => void;
    lives: number;
    gameTime: number;
    level: IGameLevel
}

interface GameProviderProps {
    children: React.ReactNode;
}

export const GameContext = createContext<GameContextProps>({
    silverCoins: 0,
    goldCoins: 0,
    incrementCoins: () => { },
    lives: 3,
    gameTime: timer,
    level: "Easy"
});

export const GameProviderScore = ({ children }: GameProviderProps) => {
    const { player, gamePlayer } = useGame();
    const retries = player ? player[1].retries : 3;
    const level = gamePlayer ? gamePlayer?.[1].level : "Easy"
    const livesLeft = retriesToLivesMap[retries];
    const gameTime = timer
    const lives = livesLeft

    const [silverCoins, setSilverCoins] = useState(0);
    const [goldCoins, setGoldCoins] = useState(0);

    const incrementCoins = (coinType: 'silver' | 'gold') => {
        if (coinType === 'silver') {
            setSilverCoins(prevCoins => prevCoins + 1);
        } else if (coinType === 'gold') {
            setGoldCoins((prevCoins) => prevCoins + 1);
        }
    };

    return (
        <GameContext.Provider
            value={{ silverCoins, goldCoins, incrementCoins, lives, gameTime, level }}
        >
            {children}
        </GameContext.Provider>
    );
};
