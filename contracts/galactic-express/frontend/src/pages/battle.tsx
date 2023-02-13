import { BattlePlayersQueue } from 'components/sections/battle-players-queue';
import { BattleWaitRegistration } from 'components/sections/battle-wait-registration';
import { useApp, useBattle } from 'app/context';
import { BattleWaitAdmin } from 'components/sections/battle-wait-admin';
import { BattleRound } from 'components/sections/battle-round';
import { BattleWinner } from 'components/sections/battle-winner';
import clsx from "clsx";
import {useEffect, useState} from "react";

//{/*{battle?.state === 'Registration' && (isAdmin ? <BattleWaitAdmin /> : <BattleWaitRegistration />)}*/}
//       {/*{battle && ['GameIsOn', 'WaitNextRound'].includes(battle.state) && rivals.length && <BattleRound />}*/}
//       {/*{battle && battle?.state === 'GameIsOver' && rivals.length && currentPlayer && <BattleWinner battle={battle} />}*/}
//       {/*{battle && Object.keys(battle.players).length > 0 && <BattlePlayersQueue />}*/}


interface RacePosition {
    xoffset: number;
}

interface RaceState {
    race1: RacePosition
    race2: RacePosition
    race3: RacePosition
    race4: RacePosition
}

export const Battle = () => {
  // const { isAdmin } = useApp();
  // const { battle, rivals, currentPlayer } = useBattle();
  const [texts, setTexts] = useState<string[]>([])
    const [state, setState] = useState<RaceState>({
        race1: { xoffset: 0},
        race2: { xoffset: 0},
        race3: { xoffset: 0},
        race4: { xoffset: 0},
    })

    function moveRacePostition(key: string, value: number) {
      const race: {[key: string]: RacePosition} = { }
        // @ts-ignore
        race[key] = { xoffset: state[key].xoffset >= 90 ? state[key].xoffset: state[key].xoffset + 10 }

      const updateState: RaceState = {
          ...state,
          ...race
        }


        setState(updateState)
    }

    useEffect(() => {
        const timer = setInterval(() => {
            setTexts(prevState => [...prevState, 'fasdfasdfsfsad']);
        }, 1000);

        return () => clearInterval(timer)

    }, [])



  return (
    <div className="flex flex-col" style={{height: '85vh'}}>
        <div><button onClick={() => moveRacePostition('race1', 15)}>moveForward_1</button></div>
        <div><button onClick={() => moveRacePostition('race2', 15)}>moveForward_2</button></div>
        <div><button onClick={() => moveRacePostition('race3', 15)}>moveForward_3</button></div>
        <div><button onClick={() => moveRacePostition('race4', 15)}>moveForward_4</button></div>
      <div><button className="bg-amber-100" onClick={() => setTexts([...texts, 'adsljalksdfjlas'])}>click</button></div>
        <div className="w-full h-1/2 border-2 border-lime-200">
          <div className="h-1/4 w-full border-2">
                <span className="text-h2" style={{
                    position: "absolute",
                    left: `${state.race1.xoffset}%`,
                }}>{"🚀"}</span>
          </div>
          <div className="h-1/4 border-2">
              <span className="text-h2" style={{
                  position: "absolute",
                  left: `${state.race2.xoffset}%`,
              }}>{"🚀"}</span>
          </div>
          <div className="h-1/4 border-2">
              <span className="text-h2" style={{
                  position: "absolute",
                  left: `${state.race3.xoffset}%`,
              }}>{"🚀"}</span>
          </div>
          <div className="h-1/4 border-2">
              <span className="text-h2" style={{
                  position: "absolute",
                  left: `${state.race4.xoffset}%`,
              }}>{"🚀"}</span>
          </div>
      </div>
      <div className="w-full h-1/2 border-2 border-lime-600">
          <div className="h-full flex flex-col overflow-auto">
              {texts.length >= 1 && texts.map(text => <span className=''>{text}</span>)}
          </div>
      </div>
    </div>
  );
};
