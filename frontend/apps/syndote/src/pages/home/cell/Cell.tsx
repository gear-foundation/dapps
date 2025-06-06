/* eslint-disable jsx-a11y/no-static-element-interactions */
/* eslint-disable jsx-a11y/click-events-have-key-events */
import { HexString } from '@polkadot/util/types';
import clsx from 'clsx';
import { FunctionComponent, SVGProps, useState } from 'react';

import { PlayerInfoState } from '@/app/utils';
import GradeSVG from '@/assets/images/icons/grade.svg?react';
import { useOutsideClick } from '@/hooks';
import { CellValues, PlayerType, Properties } from '@/types';

import styles from '../Home.module.scss';
import { Chip } from '../chip';

type Props = {
  index: number;
  players: (PlayerInfoState & PlayerType)[];
  Image: FunctionComponent<SVGProps<SVGSVGElement>> | string;
  ownership: { [key: number]: HexString } | undefined;
  properties: Properties | undefined;
  card: CellValues | undefined;
  type: string;
};

function Cell({ index, players, ownership, properties, Image, card, type }: Props) {
  const [isCardVisible, setIsCardVisible] = useState(false);
  const ref = useOutsideClick<HTMLDivElement>(() => setIsCardVisible(false));

  const getColor = (address: HexString) => players?.find((player) => player.address === address)?.color;
  const getGrade = (grade: 'Bronze' | 'Silver' | 'Gold') => {
    switch (grade) {
      case 'Gold':
        return (
          <>
            <GradeSVG />
            <GradeSVG />
            <GradeSVG />
          </>
        );
      case 'Silver':
        return (
          <>
            <GradeSVG />
            <GradeSVG />
          </>
        );
      case 'Bronze':
        return <GradeSVG />;
      default:
        return null;
    }
  };

  const chips = players
    .filter(({ position, lost }) => !lost && +position === index)
    .map(({ color }) => <Chip key={color} color={color} />);

  const isAnyChip = chips.length > 0;

  const propertyValue = properties?.[index]?.[2];

  const ownershipColor = ownership?.[index] ? getColor(ownership[index]) : undefined;

  const grade = properties?.[index]?.[1]?.[0];

  return (
    <div
      ref={ref}
      onClick={() => setIsCardVisible((prevValue) => !prevValue)}
      className={clsx(styles.a, styles[`a${index}`], ownershipColor && styles[ownershipColor], styles[type])}>
      {typeof Image === 'string' ? (
        <img src={Image} alt="" className={styles.icon} />
      ) : (
        <Image className={styles.icon} />
      )}

      {isAnyChip && <div className={styles.chips}>{chips}</div>}
      {propertyValue && <div className={styles.propertyValue}>{propertyValue}</div>}
      {grade && <div className={styles.grade}>{getGrade(grade)}</div>}

      {isCardVisible && card && (
        <div className={styles.card}>
          <header className={styles.header}>{card.heading}</header>
          <div className={styles.body}>
            <p>Increase efficiency to increase rent</p>
            <div>
              <p className={styles.infoRow}>
                Base rent
                <span className={styles.value}>{card.baseRent}</span>
              </p>
              <p className={styles.infoRow}>
                <GradeSVG />
                <span className={styles.value}>{card.bronze}</span>
              </p>
              <p className={styles.infoRow}>
                <span className={styles.gradeSVGs}>
                  <GradeSVG />
                  <GradeSVG />
                </span>

                <span className={styles.value}>{card.silver}</span>
              </p>
              <p className={styles.infoRow}>
                <span className={styles.gradeSVGs}>
                  <GradeSVG />
                  <GradeSVG />
                  <GradeSVG />
                </span>
                <span className={styles.value}>{card.gold}</span>
              </p>
            </div>
            <div>
              <p className={styles.infoRow}>
                Cell value
                <span className={styles.value}>{card.cell}</span>
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export { Cell };
