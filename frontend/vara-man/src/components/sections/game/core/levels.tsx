import { IGameLevel, IGameLevelConfig } from "@/app/types/game";
import { easyMap, mediumMap, hardMap } from './maps'

export const gameLevelConfigs: Record<IGameLevel, IGameLevelConfig> = {
    Easy: {
        speed: 1,
        map: easyMap,

    },
    Medium: {
        speed: 2,
        map: mediumMap,

    },
    Hard: {
        speed: 3,
        map: hardMap,

    },
};