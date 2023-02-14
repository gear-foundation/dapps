import {useEffect, useState} from "react";
import {RocketRace} from "../components/common/rocket-race";
import {useLounch} from 'app/context';
import {WEATHER} from 'app/consts';

import {Loader} from 'components/loaders/loader'
import {ParticipantDataType} from "../app/types/battles";

export interface RacePosition {
  id: string;
  xoffset: number; //default 5
  bgColor: string;
  fuel: number | null;
  payload: number | null;
  eventEmoji?: null | string;
}

export const Launch = () => {
  const { launch } = useLounch();


  const [texts, setTexts] = useState<string[]>([])
  const [state, setState] = useState<RacePosition[]>([])

  const currentSessionRegisteredKeys = launch && Object.keys(launch.currentSession!.registered);

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
    const updateState = [];
    if(launch && currentSessionRegisteredKeys!.length >= 1) {
      // @ts-ignore
      for(const key of currentSessionRegisteredKeys) {
        // @ts-ignore
        const { fuel, payload } = launch!.currentSession!.registered[key] as ParticipantDataType;

        const register: RacePosition = { id: key, bgColor: '#ADB2AF', fuel, payload, xoffset: 5,  }

        updateState.push(register)
      }

      // setState(prevState => [...prevState, register])
      setState(updateState)
    }
  }, [launch])

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
                    <h1 style={{ color: 'green' }}>{'↓ register'}</h1>
                  </div>
                  <div className='border-2 w-3/6 p-1'>
                    <h1 style={{ color: 'green' }}>fuel left</h1>
                  </div>
                  <div className='border-2 w-3/6 p-1'>
                    <h1 style={{ color: 'green' }}>altitude</h1>
                  </div>
                </div>
                <div className="flex flex-col overflow-auto text-center border-b">
                  {state.map((race) => {
                    return (<div className='flex flex-row' key={race.id}>
                      <span className='w-3/6 p-1'>{`${race.id.slice(1, 3)}-${race.id.slice(-3)}`}</span>
                      <span className='w-3/6 p-1'>{race.fuel}</span>
                      <span className='w-3/6 p-1'>{race.payload}</span>
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
