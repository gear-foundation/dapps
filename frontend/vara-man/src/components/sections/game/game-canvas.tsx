import React, { useCallback, useContext, useEffect, useMemo, useRef, useState } from 'react';
import { GameContext } from '@/app/context/ctx-game-score.js';
import GameEngine from './core/GameEngine';

import GameModal from './game-modal';

import style from './game.module.scss';
import { useGame } from '@/app/context/ctx-game';
import { IGameLevel, IGameLevelConfig } from '@/app/types/game';

export const gameLevelConfigs: Record<IGameLevel, IGameLevelConfig> = {
    Easy: {
        speed: 1,
        numberOfEnemies: 5,
        lives: 3,
    },
    Medium: {
        speed: 2,
        numberOfEnemies: 10,
        lives: 2,
    },
    Hard: {
        speed: 3,
        numberOfEnemies: 15,
        lives: 1,
    },
};


const GameCore = () => {
    const { incrementCoins, timer } = useContext(GameContext);

    const canvasRef = useRef(null);
    const [gameOver, setGameOver] = useState(false);
    const [isOpenModal, setOpenModal] = useState(false)

    useEffect(() => {
        const levelConfig = gameLevelConfigs["Easy"]
        const canvas = canvasRef.current;
        const gameActions = {
            incrementCoins,
            setGameOver
        };
        
        if (canvas && !gameOver) {
            const gameEngine = new GameEngine(canvas, gameActions, timer);
            gameEngine.setCanvasSize();
            gameEngine.startGameLoop();

            return () => {
                gameEngine.stopGameLoop();
            };
        }

        if (gameOver) {
            setOpenModal(true);
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [gameOver]);



    useEffect(() => {
        const handleKeyDown = (event: KeyboardEvent) => {
            const keysToPreventScroll = [37, 38, 39, 40]; // Arrow keys
            if (keysToPreventScroll.includes(event.keyCode)) {
                event.preventDefault();
            }
        };

        document.addEventListener('keydown', handleKeyDown);

        return () => {
            document.removeEventListener('keydown', handleKeyDown);
        }
    }, [])

    return (
        <>
            <div className={style.canvas}>
                {isOpenModal && <GameModal setOpenModal={setOpenModal} />}
                <canvas ref={canvasRef} id="gameCanvas" />
            </div>
        </>
    );
};

export default GameCore;
