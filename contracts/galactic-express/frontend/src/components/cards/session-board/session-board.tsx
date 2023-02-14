import styles from './board.module.scss';
import clsx from 'clsx';
import { useLounch } from 'app/context';
import { WEATHER } from 'app/consts'

export const SessionBoard = () => {

  const { launch } = useLounch();
  console.log(launch)

  const players = Object.keys(launch!.currentSession!.registered).length;
  console.log(players)

  return (

    <div className={clsx('w-1/3', styles.board)}>
      <h2 className={styles.session}>Session #{launch?.sessionId}</h2>
      <div className="flex flex-col">
        <div className="flex flex-row">
          <div className="m-1 flex-none w-30 text-left">
            <span>Altitude:</span>
          </div>
          <div className="m-1 grow">
            <span>{launch?.currentSession?.altitude}</span>
          </div>
        </div>
        <div className="flex flex-row">
          <div className="m-1 flex-none w-30 text-left">
            <span>Weather:</span>
          </div>
          <div className="m-1 grow">
            <span>{WEATHER[launch!.currentSession!.weather]}</span>
          </div>
        </div>
        <div className="flex flex-row">
          <div className="m-1 flex-none w-30 text-left">
            <span>Fuel Price:</span>
          </div>
          <div className="m-1 grow">
            <span>{launch?.currentSession?.fuelPrice}</span>
          </div>
        </div>
        <div className="flex flex-row">
          <div className="m-1 flex-none text-left w-30">
            <span>Reward:</span>
          </div>
          <div className="m-1 grow">
            <span>{launch?.currentSession?.payloadValue}</span>
          </div>
        </div>
        <div className="flex flex-col panel2">
          <p className="blink reg">{launch?.state}</p>
          <p>Rockets: {players}/4</p>
        </div>
      </div>
    </div>
  );
};
