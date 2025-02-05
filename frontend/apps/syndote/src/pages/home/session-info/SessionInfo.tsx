import { useAccount, useAlert, useApi, useBalanceFormat, withoutCommas, getVaraAddress } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { Players } from '@/types';
import { useSyndoteMessage } from '@/hooks/metadata';
import VaraSVG from '@/assets/images/icons/vara-coin.svg?react';
import TVaraSVG from '@/assets/images/icons/tvara-coin.svg?react';
import UserSVG from '@/assets/images/icons/ic-user-small-24.svg?react';
import CopySVG from '@/assets/images/icons/copy-text.svg?react';
import RemovePlayerSVG from '@/assets/images/icons/remove-player.svg?react';
import styles from './SessionInfo.module.scss';
import { stringShorten } from '@polkadot/util';
import { GameDetails } from '@/components/layout/game-details';
import clsx from 'clsx';
import { HexString } from '@gear-js/api';
import { copyToClipboard } from '@/utils';

type Props = {
  entryFee: string | null;
  players: Players;
  adminId: string;
};

function SessionInfo({ entryFee, players, adminId }: Props) {
  const { api } = useApi();
  const { account } = useAccount();
  const alert = useAlert();
  const { isMeta, sendMessage } = useSyndoteMessage();
  const { getFormattedBalanceValue } = useBalanceFormat();
  const VaraSvg = api?.registry.chainTokens[0].toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />;

  const handleCopy = (value: string) => {
    copyToClipboard({ alert, value });
  };

  const items = [
    {
      name: 'Entry fee',
      value: (
        <>
          {VaraSvg} {entryFee ? getFormattedBalanceValue(Number(withoutCommas(entryFee))).toFormat(2) : 0} VARA
        </>
      ),
      key: '1',
    },
    {
      name: 'Players already joined the game',
      value: (
        <>
          <UserSVG /> {players.length}
          <span className={styles.fromAllPlayers}>/4</span>
        </>
      ),
      key: '2',
    },
    {
      name: (
        <span>
          Program address (<span className={styles.markedAddress}>{stringShorten(getVaraAddress(adminId), 4)}</span>)
        </span>
      ),
      value: (
        <Button
          color="transparent"
          icon={CopySVG}
          text="Copy"
          className={styles.copyButton}
          onClick={() => handleCopy(getVaraAddress(adminId))}
        />
      ),
      key: '3',
    },
  ];
  const isAdmin = adminId === account?.decodedAddress;

  const removePlayer = (playerId: HexString) => {
    if (!isMeta) {
      return;
    }

    const payload = {
      DeletePlayer: {
        playerId,
      },
    };

    sendMessage({
      payload,
    });
  };

  return (
    <>
      <GameDetails items={items} className={{ item: styles.gameDetailsItem }} />
      <ul className={styles.playersContainer}>
        {players.map((player) => (
          <li
            key={player[1].ownerId}
            className={clsx(
              styles.playerItem,
              player[1].ownerId === account?.decodedAddress && styles.playerItemAdmin,
              isAdmin && player[1].ownerId !== account?.decodedAddress && styles.playerItemForAdmin,
            )}>
            <span>
              {stringShorten(getVaraAddress(player[1].ownerId), 4)}{' '}
              {player[1].ownerId === account?.decodedAddress ? <span className={styles.playerLabel}>(you)</span> : ''}
            </span>
            {isAdmin && player[1].ownerId !== account?.decodedAddress && (
              <Button color="transparent" icon={RemovePlayerSVG} onClick={() => removePlayer(player[1].ownerId)} />
            )}
          </li>
        ))}
      </ul>
    </>
  );
}

export { SessionInfo };
