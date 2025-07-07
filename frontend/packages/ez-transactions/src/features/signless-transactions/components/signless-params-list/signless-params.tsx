import { JSX } from 'react';

import styles from './signless-params.module.css';

type Props = {
  params: {
    heading: string;
    value: JSX.Element | string | number;
  }[];
};

function SignlessParams({ params }: Props) {
  const renderParams = () =>
    params.map(
      ({ heading, value }) =>
        value && (
          <li className={styles.summaryItem} key={heading}>
            <h4 className={styles.heading}>{heading}</h4>
            <div className={styles.separator} />
            <p className={styles.value}>{value}</p>
          </li>
        ),
    );

  return <ul className={styles.summary}>{renderParams()}</ul>;
}

export { SignlessParams };
