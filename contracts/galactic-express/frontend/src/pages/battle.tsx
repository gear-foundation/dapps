import {useEffect, useState} from "react";
import {RocketRace} from "../components/common/rocket-race";

export enum RaceStatus {
    Registration = 'Registration',
    GameIsOver = 'GameIsOver',
    GameIsOn = 'GameIsOn'
}

interface RacePosition {
    id: string;
    xoffset: number;
    backgroundColor: string;
    eventEmoji?:  null  | string;
    status: RaceStatus,
}

export const Battle = () => {
  // const { isAdmin } = useApp();
  // const { battle, rivals, currentPlayer } = useBattle();
    const [texts, setTexts] = useState<string[]>([])
    const [state, setState] = useState<RacePosition[]>([
        { id: '1', xoffset: 0, backgroundColor: '#19E676', status: RaceStatus.Registration },
        { id: '2', xoffset: 0, backgroundColor: '#1751CB', status: RaceStatus.Registration },
        { id: '3', xoffset: 0, backgroundColor: '#DD26C5', status: RaceStatus.Registration },
        { id: '4', xoffset: 0, backgroundColor: '#ADB2AF', status: RaceStatus.Registration },
    ])

    function moveRacePostition(id: string, step: number): void {
        const updateState = state.map(rocket => {
            if(rocket.id === id && rocket.xoffset <= 90) {
                return { ...rocket, xoffset: rocket.xoffset + 15}
            } else {
                return rocket
            }
        })

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
        {/*<div><button onClick={() => moveRacePostition('1', 15)}>moveForward_1</button></div>*/}
      <div className="w-full h-1/2 border-2 border-lime-200">
            { state.map(rocket =>  RocketRace(rocket)) }
      </div>
      <div className="w-full h-1/2 border-2 border-lime-600">
          <div className="h-full flex flex-col overflow-auto">
              {texts.length >= 1 && texts.map(text => <span className=''>{text}</span>)}
          </div>
      </div>
    </div>
  );
};
