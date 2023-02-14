import { useEffect, useState } from "react";
import { RocketRace } from "../components/common/rocket-race";
import { useLounch } from 'app/context';
import { WEATHER } from 'app/consts';
import { Loader } from 'components/loaders/loader'

export enum RaceStatus {
  Registration = 'Registration',
  GameIsOver = 'GameIsOver',
  GameIsOn = 'GameIsOn'
}

export enum WeatherRisk {

}

interface RacePosition {
  id: string;
  xoffset: number;
  backgroundColor: string;
  eventEmoji?: null | string;
  status: RaceStatus,
}

export const Launch = () => {
  const { launch } = useLounch();


  console.log(launch)

  const [texts, setTexts] = useState<string[]>([])
  const [state, setState] = useState<RacePosition[]>([
    { id: '1', xoffset: 5, backgroundColor: '#19E676', status: RaceStatus.Registration },
    { id: '2', xoffset: 5, backgroundColor: '#1751CB', status: RaceStatus.Registration },
    { id: '3', xoffset: 5, backgroundColor: '#DD26C5', status: RaceStatus.Registration },
    { id: '4', xoffset: 5, backgroundColor: '#ADB2AF', status: RaceStatus.Registration },
  ])

  function moveRacePostition(id: string, step: number): void {
    const updateState = state.map(rocket => {
      if (rocket.id === id && rocket.xoffset <= 85) {
        return { ...rocket, xoffset: rocket.xoffset + 8 }
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
    <div className="flex flex-col items-center w-11/12 mx-auto" style={{ height: '85vh' }}>
      {!launch ? (
        <Loader />
      ) : (
        <>
          <div><button onClick={() => moveRacePostition('1', 15)}>moveForward_1</button></div>
          <div className="w-full h-1/2 border-b-gray-900">
            {state.map(rocket => RocketRace(rocket))}
          </div>
          <div className="flex flex-row w-full h-1/2 logs">
            <div className="w-9/12 flex flex-col overflow-auto border-2 p-1">
              {texts.length >= 1 && texts.map(text => {
                return <div><span className='text-green-400'>{'>'}</span> <span>{text}</span></div>
              })}
            </div>
            <div className="flex flex-col w-1/4 text-center">
              <div>
                <div className="flex flex-row">
                  <div className='border-2 w-3/6 p-1'>
                    <h1 style={{ color: 'green' }}>{'↓ position'}</h1>
                  </div>
                  <div className='border-2 w-3/6 p-1'>
                    <h1 style={{ color: 'green' }}>fuel left</h1>
                  </div>
                  <div className='border-2 w-3/6 p-1'>
                    <h1 style={{ color: 'green' }}>altitude</h1>
                  </div>
                </div>
                <div className="flex flex-col overflow-auto text-center border-b">
                  {state.map(race => {
                    return (<div className='flex flex-row'>
                      <span className='w-3/6 p-1'>{race.id}</span>
                      <span className='w-3/6 p-1'>{race.status}</span>
                      <span className='w-3/6 p-1'>{race.xoffset}</span>
                    </div>)
                  })}
                </div>
              </div>
              <div className='flex flex-col '>
                <div className='border-b'>{`weather`}</div>
                <span className='text-xl'>{WEATHER[launch!.currentSession!.weather]}</span>
              </div>
            </div>
          </div>
        </>
      )}



    </div>
  );
};
