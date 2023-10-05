import { TRAITS, WEATHERS } from '../../consts';
import styles from './Traits.module.scss';

type Props = {
  altitude: string;
  weather: string;
  fuelPrice: string;
  reward: string;
};

function Traits({ altitude, weather, fuelPrice, reward }: Props) {
  const getTraits = () => {
    // same order as in TRAITS
    const traitValues = [altitude, WEATHERS[weather as keyof typeof WEATHERS].name, fuelPrice, reward];

    return TRAITS.map(({ heading, SVG }, index) => (
      <li key={heading} className={styles.trait}>
        <SVG className={styles.svg} />
        <h3 className={styles.heading}>{heading}</h3>
        <p className={styles.subheading}>{traitValues[index]}</p>
      </li>
    ));
  };

  return <ul className={styles.traits}>{getTraits()}</ul>;
}

export { Traits };
