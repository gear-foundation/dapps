import { useEffect, useState } from "react";
import { RocketRace } from "../components/common/rocket-race";
import { useLounch } from 'app/context';
import { WEATHER } from 'app/consts';

import { Loader } from 'components/loaders/loader'
import { EventData, ParticipantDataType, SessionStatus } from "../app/types/battles";

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

  console.log(launch)

  const [readLogs, setReadLogs] = useState<{ sessionNum: number, event: EventData }[]>([])
  const [statusSessionRace, setStatusSessionRace] = useState<SessionStatus>(SessionStatus.INIT)
  const [state, setState] = useState<RacePosition[]>([])
  const [logs, setLogs] = useState<{ sessionNum: number, events: EventData[] }[]>([])
  const [count, setCount] = useState(0);

  const currentSessionRegisteredKeys = launch && Object.keys(launch.currentSession!.registered);

  useEffect(() => {
    if (launch && currentSessionRegisteredKeys!.length >= 1) {
      const updateState = [];
      // @ts-ignore
      for (const key of currentSessionRegisteredKeys) {
        // @ts-ignore
        const { fuel, payload } = launch!.currentSession!.registered[key] as ParticipantDataType;

        const register: RacePosition = { id: key, bgColor: '#ADB2AF', fuel, payload, xoffset: 5, }

        updateState.push(register)
      }

      // setState(prevState => [...prevState, register])
      setState(updateState)
    }

    if (launch && launch.state === SessionStatus.REGISTRATION) {
      setStatusSessionRace(SessionStatus.REGISTRATION)
    }

    if (launch && launch.state === SessionStatus.SESSION_IS_OVER) {
      const keysEvents = Object.keys(launch.events)
      let state = []
      //Логи
      for (const sessionKey of keysEvents) {
        // @ts-ignore
        const sessionEventData = launch.events[sessionKey];
        const createSessionEventLogs = { sessionNum: Number(sessionKey), events: sessionEventData as EventData[] }
        state.push(createSessionEventLogs)
      }
      // @ts-ignore
      setLogs(state)
    }
  }, [launch])

  useEffect(() => {
    let counter = count;
    let interval: any

    if (launch && launch.state === SessionStatus.SESSION_IS_OVER && logs.length >= 1) {
      interval = setInterval(() => {
        if (logs && counter >= logs.length) {
          clearInterval(interval);
        } else {
          setCount((count) => count + 1);
          // @ts-ignore
          const dataLogs = logs[counter];
          let setLogsList = [];
          let raceSessionState = [];

          for (const event of dataLogs.events) {
            setLogsList.push({ sessionNum: dataLogs.sessionNum, event })
            raceSessionState.push({
              xoffset: getOffsetBySession(dataLogs.sessionNum),
              id: event.participant,
              payload: event.payload,
              fuel: event.fuelLeft,
            } as RacePosition)
          }
          setReadLogs([...readLogs, ...setLogsList])
          let raceSessionStatewWithEmptyTemplate = [...raceSessionState, ...getEmptyTemplateByEventsLength(dataLogs.events.length)]
          setState(raceSessionStatewWithEmptyTemplate)

          counter++;
        }
      }, 2000);
    }

    return () => clearInterval(interval);
  }, [count, launch, logs]);

  return (
    <div className="flex flex-col items-center w-10/12 mx-auto" style={{ height: '85vh' }}>
      {!launch ? (
        <Loader />
      ) : (
        <>
          <div className="w-full h-1/3 border-b-gray-900">
            {state.map(rocket => RocketRace({ ...rocket, sessionStatus: statusSessionRace }))}
          </div>
          <div className="flex flex-row w-full h-2/3 logs">
            <div className="w-9/12 flex flex-col overflow-auto border-2 p-1">
              {readLogs.length >= 1 && readLogs.map(logs => {
                return (<div>
                  <div>
                    <span className='arrow'>{'>'}</span>
                    <span className='sessionId'>{`Session: ${logs.sessionNum}`}</span>
                  </div>
                  <div>
                    <span className='arrow'>{'>'}</span>
                    <span>Player: </span>
                    <span className='player'>{`${logs.event.participant.slice(1, 5)}...${logs.event.participant.slice(-4)}`}</span>
                  </div>
                  <div>
                    <span className='arrow'>{'>'}</span>
                    {`Data: Alive: ${logs.event.alive}, Fuel left: ${logs.event.fuelLeft}, Last Altitude: ${logs.event.lastAltitude} Payload: ${logs.event.payload}, Halt: ${logs.event.halt}`}
                  </div>
                </div>)
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
                    <h1 style={{ color: 'green' }}>Payload</h1>
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

function getOffsetBySession(session: number): number {
  if (session === 0) return 10
  if (session === 1) return 45
  if (session === 2) return 85

  return 10
}

function getEmptyTemplateByEventsLength(eventsLength: number): RacePosition[] {
  let res = []
  const emptyTemplate: RacePosition = { id: '', fuel: 0, payload: 0, xoffset: 10, bgColor: '#7b0015' }
  if (eventsLength === 2) {
    res.push(...[emptyTemplate, emptyTemplate])
  }
  if (eventsLength === 3) {
    res.push(...[emptyTemplate])
  }

  return res
}
