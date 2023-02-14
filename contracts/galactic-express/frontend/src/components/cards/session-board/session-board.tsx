import styles from './board.module.scss';
import clsx from 'clsx';
import { useLounch } from 'app/context';

export const SessionBoard = () => {

  return (

    <div className={clsx('w-1/3', styles.board)}>
      <h2 className={styles.session}>Session #33</h2>
      <div className="flex flex-col">
        <div className="flex flex-row">
          <div className="m-1 flex-none w-20 text-left">
            <span>Distance:</span>
          </div>
          <div className="m-1 grow">
            <span>250 000</span>
          </div>
        </div>
        <div className="flex flex-row">
          <div className="m-1 flex-none w-20 text-left">
            <span>Risk:</span>
          </div>
          <div className="m-1 grow">
            <span>Cloudy</span>
          </div>
        </div>
        <div className="flex flex-row">
          <div className="m-1 flex-none w-20 text-left">
            <span>Price:</span>
          </div>
          <div className="m-1 grow">
            <span>10 000</span>
          </div>
        </div>
        <div className="flex flex-row">
          <div className="m-1 flex-none text-left w-20">
            <span>Reward:</span>
          </div>
          <div className="m-1 grow">
            <span>345</span>
          </div>
        </div>
      </div>
    </div>
  );
};
