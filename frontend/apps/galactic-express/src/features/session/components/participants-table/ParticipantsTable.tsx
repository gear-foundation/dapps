import { Fragment } from 'react';
import { cx } from '@/utils';
import { shortenString } from '@/features/session/utils';
import { decodeAddress } from '@gear-js/api';
import { Button } from '@gear-js/vara-ui';
import { useDeletePlayerMessage } from '@/app/utils';
import styles from './ParticipantsTable.module.scss';

interface TableData {
  id: string;
  playerAddress: string;
  playerName: string;
}

type Props = {
  data: TableData[];
  userAddress: string;
  isUserAdmin: boolean;
};

function ParticipantsTable({ data, userAddress, isUserAdmin }: Props) {
  const { deletePlayerMessage } = useDeletePlayerMessage();
  const isYourAddress = (address: string) => address === userAddress;

  const modifiedData: TableData[] = [
    ...data.filter((item) => isYourAddress(item.playerAddress)),
    ...data.filter((item) => !isYourAddress(item.playerAddress)),
  ];

  const handleDeletePlayer = (playerId: string) => {
    deletePlayerMessage({ playerId: decodeAddress(playerId) });
  };

  return (
    <table className={cx(styles.table)}>
      {modifiedData && (
        <>
          <thead>
            <tr>
              {modifiedData[0] && (
                <>
                  <td className={cx(styles.headTd)}>#</td>
                  {Object.keys(modifiedData[0]).map(
                    (cellName: string) =>
                      cellName !== 'id' && (
                        <Fragment key={modifiedData[0].id + cellName}>
                          <td className={cx(styles.headTd)}>{cellName}</td>
                        </Fragment>
                      ),
                  )}
                </>
              )}
            </tr>
          </thead>
          <tbody className={cx(styles.body)}>
            {modifiedData?.map((row, rowIndex) => (
              <tr
                key={row.id}
                className={cx(styles.bodyTr, isYourAddress(row.playerAddress) ? styles.bodyTrWithYourAddress : '')}>
                <td className={cx(styles.bodyTd, styles.bodyTdIndex)}>{rowIndex + 1}</td>
                {Object.keys(row).map(
                  (cellName) =>
                    cellName !== 'id' && (
                      <Fragment key={cellName}>
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
                      </Fragment>
                    ),
                )}
                {isUserAdmin && (
                  <td className={cx(styles.bodyTd, styles.removeTd)}>
                    <Button
                      color="transparent"
                      className={styles.removeButton}
                      onClick={() => handleDeletePlayer(row.id)}>
                      {!isYourAddress(row.playerAddress) && 'Remove Player'}
                    </Button>
                  </td>
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
