import { useLounch } from 'app/context';
import { useEffect, useState } from 'react';
import styles from './calc.module.scss';
import clsx from 'clsx';

export const LauncheCalc = () => {

  const { launch } = useLounch();

  const [fuel, setFuel] = useState<number>(50);
  const [payload, setPayload] = useState<number>(50);
  const [probability, setProbability] = useState<number>(0);

  // This function is called when the first range slider changes
  const changeFuel = (event: any) => {
    setFuel(event.target.value);
  };

  // This function is called when the second range slider changes
  const changePayload = (event: any) => {
    setPayload(event.target.value);
  };


  useEffect(() => {
    if (launch) {
      const weather = launch.currentSession?.weather;

      const prob = 97 / 100 * (95 - weather!) / 100 * (90 - weather!) / 100;
      setProbability(prob);

      if (payload >= 80) {
        const prob = 97 / 100 * (85 - 2 * weather!) / 100 * (90 - weather!) / 100;
        setProbability(prob);
      }

      if (fuel >= 80) {
        const prob = (87 - 2 * weather!) / 100 * (95 - weather!) / 100 * (90 - weather!) / 100;
        setProbability(prob);
      }

      if (fuel >= 80 && payload >= 80) {
        const prob = (87 - 2 * weather!) / 100 * (85 - 2 * weather!) / 100 * (90 - weather!) / 100;
        setProbability(prob);
      }


    }
    console.log(fuel)
    console.log(payload)

  }, [fuel, payload, launch])



  return (
    <div className={clsx(styles.lcalc)}>
      <h2>*** Calculation block ***</h2>
      <div className="range flex flex-col">
        <label>Fuel:<span>{fuel}%</span></label>
        <input
          type='range'
          onChange={changeFuel}
          min={1}
          max={100}
          step={1}
          value={fuel}
        ></input>
      </div>
      <div className="range flex flex-col">
        <label>Payload: <span>{payload}%</span></label>
        <input
          type='range'
          onChange={changePayload}
          min={1}
          max={100}
          step={1}
          value={payload}
          className='custom-slider'
        ></input>
      </div>

      <p style={{ color: 'green' }}>here is calculation / {(probability * 100).toFixed(2)}  %/</p>
    </div>
  );
};
