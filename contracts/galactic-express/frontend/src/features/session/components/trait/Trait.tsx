import { SVGComponent } from 'types';
import styles from './Trait.module.scss';

type Props = {
  SVG: SVGComponent;
  heading: string;
  subheading: string;
};

function Trait({ SVG, heading, subheading }: Props) {
  return (
    <li className={styles.trait}>
      <SVG className={styles.svg} />
      <h3 className={styles.heading}>{heading}</h3>
      <p className={styles.subheading}>{subheading}</p>
    </li>
  );
}

export { Trait };
