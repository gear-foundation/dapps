import { Fragment } from 'react';
import { cx } from 'utils';
import { shortenString } from 'features/session/utils';
import styles from './ParticipantsTable.module.scss';
import { Button } from '@gear-js/vara-ui';

interface TableData {
  id: string;
  playerAddress: string;
}

type Props = {
  data: TableData[];
  userAddress: string;
  isUserAdmin: boolean;
};

function ParticipantsTable({ data, userAddress, isUserAdmin }: Props) {
  const isYourAddress = (address: string) => address === userAddress;

  const modifiedData: TableData[] = [
    ...data.filter((item) => isYourAddress(item.playerAddress)),
    ...data.filter((item) => !isYourAddress(item.playerAddress)),
  ];

  return (
    <table className={cx(styles.table)}>
      {modifiedData && (
        <>
          <thead>
            <tr>
              {modifiedData[0] &&
                Object.keys(modifiedData[0]).map(
                  (cellName: string) =>
                    cellName !== 'id' && (
                      <Fragment key={modifiedData[0].id}>
                        <td className={cx(styles.headTd)}>#</td>
                        <td className={cx(styles.headTd)}>{cellName}</td>
                      </Fragment>
                    ),
                )}
            </tr>
          </thead>
          <tbody className={cx(styles.body)}>
            {modifiedData?.map((row, rowIndex) => (
              <tr
                key={row.id}
                className={cx(styles.bodyTr, isYourAddress(row.playerAddress) ? styles.bodyTrWithYourAddress : '')}>
                {Object.keys(row).map(
                  (cellName) =>
                    cellName !== 'id' && (
                      <Fragment key={cellName}>
                        <td className={cx(styles.bodyTd, styles.bodyTdIndex)}>{rowIndex + 1}</td>
                        <td className={cx(styles.bodyTd, styles[`bodyTd${cellName}`])}>
                          {cellName === 'playerAddress' ? (
                            <>
                              {shortenString(row[cellName as keyof TableData], 4)}
                              {isYourAddress(row[cellName]) && (
                                <span className={cx(styles.yourAddressSpan)}> (You)</span>
                              )}
                            </>
                          ) : (
                            row[cellName as keyof TableData]
                          )}
                        </td>
                        {isUserAdmin && (
                          <td className={cx(styles.bodyTd, styles.removeTd)}>
                            <Button color="transparent" className={styles.removeButton}>
                              {!isYourAddress(row.playerAddress) && 'Remove Player'}
                            </Button>
                          </td>
                        )}
                      </Fragment>
                    ),
                )}
              </tr>
            ))}
          </tbody>
        </>
      )}
    </table>
  );
}

export { ParticipantsTable };
