import { useEffect, useState } from "react";
import { RocketRace } from "../components/common/rocket-race";
import { useLounch } from 'app/context';
import { WEATHER } from 'app/consts';

import { Loader } from 'components/loaders/loader'
import { EventData, ParticipantDataType, SessionStatus } from "../app/types/battles";
import { logger } from "@polkadot/util";

export interface RacePosition {
  id: string;
  xoffset: number; //default 5
  bgColor: string;
  fuel: number | null;
  payload: number | null;
  eventEmoji?: null | string;
}

let events = {
  "0": [
    {
      "participant": "0x98ac3a9e3fb7256e36722a38a34046267776ab935130a073c8cc58ba8892266a",
      "alive": true,
      "fuelLeft": "53",
      "lastAltitude": "4,421",
      "payload": "79",
      "halt": null
    },
    {
      "participant": "0xaaf5f429550085819a1fe69fcf79cfed35752a142bd13fc8bb17c1d47615fa29",
      "alive": true,
      "fuelLeft": "34",
      "lastAltitude": "4,421",
      "payload": "50",
      "halt": null
    },
    {
      "participant": "0xc4406937dd46aad223aae39dd83981807fa24aff2dd1af72f795c9f1627b0c71",
      "alive": false,
      "fuelLeft": "40",
      "lastAltitude": "4,421",
      "payload": "60",
      "halt": "SeparationFailure"
    },
    {
      "participant": "0xe88e9832faf94c159f962fd22e0cc4d5f2552e997cd4961bbca10d488c97cf57",
      "alive": true,
      "fuelLeft": "54",
      "lastAltitude": "4,421",
      "payload": "79",
      "halt": null
    }
  ],
  "1": [
    {
      "participant": "0x98ac3a9e3fb7256e36722a38a34046267776ab935130a073c8cc58ba8892266a",
      "alive": true,
      "fuelLeft": "27",
      "lastAltitude": "8,842",
      "payload": "79",
      "halt": null
    },
    {
      "participant": "0xaaf5f429550085819a1fe69fcf79cfed35752a142bd13fc8bb17c1d47615fa29",
      "alive": true,
      "fuelLeft": "18",
      "lastAltitude": "8,842",
      "payload": "50",
      "halt": null
    },
    {
      "participant": "0xe88e9832faf94c159f962fd22e0cc4d5f2552e997cd4961bbca10d488c97cf57",
      "alive": true,
      "fuelLeft": "28",
      "lastAltitude": "8,842",
      "payload": "79",
      "halt": null
    }
  ],
  "2": [
    {
      "participant": "0x98ac3a9e3fb7256e36722a38a34046267776ab935130a073c8cc58ba8892266a",
      "alive": true,
      "fuelLeft": "1",
      "lastAltitude": "13,263",
      "payload": "79",
      "halt": null
    },
    {
      "participant": "0xaaf5f429550085819a1fe69fcf79cfed35752a142bd13fc8bb17c1d47615fa29",
      "alive": true,
      "fuelLeft": "2",
      "lastAltitude": "13,263",
      "payload": "50",
      "halt": null
    },
    {
      "participant": "0xe88e9832faf94c159f962fd22e0cc4d5f2552e997cd4961bbca10d488c97cf57",
      "alive": false,
      "fuelLeft": "2",
      "lastAltitude": "13,263",
      "payload": "79",
      "halt": "Asteroid"
    }
  ],
  "3": [
    {
      "participant": "0x98ac3a9e3fb7256e36722a38a34046267776ab935130a073c8cc58ba8892266a",
      "alive": true,
      "fuelLeft": "1",
      "lastAltitude": "13,263",
      "payload": "79",
      "halt": null
    },
    {
      "participant": "0xaaf5f429550085819a1fe69fcf79cfed35752a142bd13fc8bb17c1d47615fa29",
      "alive": true,
      "fuelLeft": "2",
      "lastAltitude": "13,263",
      "payload": "50",
      "halt": null
    },
    {
      "participant": "0xe88e9832faf94c159f962fd22e0cc4d5f2552e997cd4961bbca10d488c97cf57",
      "alive": false,
      "fuelLeft": "2",
      "lastAltitude": "13,263",
      "payload": "79",
      "halt": "Asteroid"
    }
  ],
  "4": [
    {
      "participant": "0x98ac3a9e3fb7256e36722a38a34046267776ab935130a073c8cc58ba8892266a",
      "alive": true,
      "fuelLeft": "1",
      "lastAltitude": "13,263",
      "payload": "79",
      "halt": null
    },
    {
      "participant": "0xaaf5f429550085819a1fe69fcf79cfed35752a142bd13fc8bb17c1d47615fa29",
      "alive": true,
      "fuelLeft": "2",
      "lastAltitude": "13,263",
      "payload": "50",
      "halt": null
    },
    {
      "participant": "0xe88e9832faf94c159f962fd22e0cc4d5f2552e997cd4961bbca10d488c97cf57",
      "alive": false,
      "fuelLeft": "2",
      "lastAltitude": "13,263",
      "payload": "79",
      "halt": "Asteroid"
    }
  ]
}

export const Launch = () => {
  const { launch } = useLounch();
  const colors = ['#19a6a1', '#155263', '#dd344b', '#43ab92'];

  console.log(launch)

  const [readLogs, setReadLogs] = useState<{ sessionNum: number, event: EventData }[]>([])
  const [statusSessionRace, setStatusSessionRace] = useState<SessionStatus>(SessionStatus.INIT)
  const [state, setState] = useState<RacePosition[]>([])
  const [logs, setLogs] = useState<{ sessionNum: number, events: EventData[] }[]>([])
  const [count, setCount] = useState(0);
  const [animationState, setAnimationState] = useState([])

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
    console.log('__________>', logs)

    if (launch && launch.state === SessionStatus.SESSION_IS_OVER && logs.length >= 1) {
      interval = setInterval(() => {
        if (logs && counter >= logs.length) {
          clearInterval(interval);
        } else {
          setCount((count) => count + 1);
          // @ts-ignore
          const dataLogs = logs[counter];
          let setLogsList = [];

          for (const event of dataLogs.events) {
            setLogsList.push({ sessionNum: dataLogs.sessionNum, event })
            // setReadLogs([...readLogs, { sessionNum: dataLogs.sessionNum, event }])
          }

          setReadLogs([...readLogs, ...setLogsList])


          counter++;
        }
      }, 1000);
    }

    return () => clearInterval(interval);
  }, [count, launch, logs]);

  useEffect(() => {
    // const timer = setInterval(() => {
    //   setTexts(prevState => [...prevState, 'fasdfasdfsfsad']);
    // }, 1000);
    //
    // return () => clearInterval(timer)

    // if(launch && launch.sessionId === 4 && launch.state === SessionStatus.SESSION_IS_OVER) {
    //   setInterval(() => {
    //
    //   }, 1000)
    // }

  }, [launch])

  return (
    <div className="flex flex-col items-center w-11/12 mx-auto" style={{ height: '85vh' }}>
      {!launch ? (
        <Loader />
      ) : (
        <>
          <div className="w-full h-1/2 border-b-gray-900">
            {state.map(rocket => RocketRace({ ...rocket, sessionStatus: statusSessionRace }))}
          </div>
          <div className="flex flex-row w-full h-1/2 logs">
            <div className="w-9/12 flex flex-col overflow-auto border-2 p-1">
              {readLogs.length >= 1 && readLogs.map(logs => {
                console.log('__________________>', logs.event)

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
                    {`Data###: Alive: ${logs.event.alive}, Fuel left: ${logs.event.fuelLeft}, Last Altitude: ${logs.event.lastAltitude} Payload: ${logs.event.payload}, Halt: ${logs.event.halt}`}
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

// function animationRocketRace(state: { sessionNum: number, events: EventData[] }) {
//   return (
//       <div className="w-full h-1/2 border-b-gray-900">
//         {state.map(rocket => RocketRace({...rocket, sessionStatus: statusSessionRace}))}
//       </div>
//   )
// }
