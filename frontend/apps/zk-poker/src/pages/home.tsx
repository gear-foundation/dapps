import { useAccount } from '@gear-js/react-hooks';
import { Button as VaraButton } from '@gear-js/vara-ui';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { CreateGameIllustration, EditIcon, JoinGameIllustration, PointsIcon } from '@/assets/images';
import { Avatar, Banner, EditProfileModal, Footer, Header, MenuButton, Stats, Balance } from '@/components';
import { useUserName } from '@/features/game/hooks';
import { useGetPlayerByIdQuery } from '@/features/game/queries';
import {
  useEventLobbyCreatedSubscription,
  useEventLobbyDeletedSubscription,
  useGetBalanceQuery,
  useLobbiesQuery,
} from '@/features/game/sails';
import { ClaimPtsButton } from '@/features/pts';
import { useVaranWallet } from '@/features/wallet';

import styles from './home.module.scss';

export default function Home() {
  useVaranWallet();

  const navigate = useNavigate();
  const { balance, refetch: refetchPtsBalance } = useGetBalanceQuery();
  const { userName, setUserName } = useUserName();
  const { account } = useAccount();

  const [isEditProfileModalOpen, setIsEditProfileModalOpen] = useState(false);

  const onEditProfile = () => {
    setIsEditProfileModalOpen(true);
  };

  const { lobbies, refetch: refetchLobbies } = useLobbiesQuery();

  const onLobbyChanged = () => {
    void refetchLobbies();
  };

  useEventLobbyCreatedSubscription({ onData: onLobbyChanged });
  useEventLobbyDeletedSubscription({ onData: onLobbyChanged });

  const lobbiesCount = lobbies?.length || 0;

  const { data: playerData } = useGetPlayerByIdQuery(account?.decodedAddress);
  const { gamesToday, wins, games } = playerData?.playerById || { gamesToday: 0, wins: 0, games: 0 };

  const stats = [
    { value: games ? `${Math.round((wins / games) * 100)}%` : '-', label: 'Your Winrate' },
    { value: games, label: 'Total Hands Played' },
    { value: gamesToday, label: 'Hands Played Today' },
  ];

  const handleProfileSave = (name: string) => {
    setUserName(name);
    setIsEditProfileModalOpen(false);
  };

  const formattedBalance = balance?.toLocaleString('en-US') || '0';
  const ptsBalance = balance ? Number(balance) : undefined;

  return (
    <>
      <Header>
        <Balance value={formattedBalance} unit="PTS" SVG={PointsIcon} />
      </Header>

      <div className={styles.container}>
        <Banner title="ZK-powered Poker" subtitle="Built on Gear Protocol using ZK tech." />
        <div className={styles.buttons}>
          <MenuButton
            title="Join game"
            subtitle={`${lobbiesCount} ${lobbiesCount === 1 ? 'room' : 'rooms'}`}
            onClick={() => {
              navigate(ROUTES.ROOMS);
            }}
            illustration={JoinGameIllustration}
          />
          <MenuButton
            title="Create game"
            subtitle="~15 VARA"
            onClick={() => {
              navigate(ROUTES.CREATE_GAME);
            }}
            illustration={CreateGameIllustration}
          />
        </div>
        <h3 className={styles.welcome}>
          Welcome, <Avatar size="sm" className={styles.avatar} address={account?.decodedAddress} /> {userName}{' '}
          <VaraButton
            color="transparent"
            icon={EditIcon}
            onClick={onEditProfile}
            className={styles.editIcon}
            size="x-small"
          />
        </h3>
        <Stats items={stats} />

        <ClaimPtsButton onSuccess={refetchPtsBalance} className={styles.claim} ptsBalance={ptsBalance} />

        <Footer />

        {isEditProfileModalOpen && (
          <EditProfileModal
            userName={userName}
            onClose={() => setIsEditProfileModalOpen(false)}
            onSave={handleProfileSave}
          />
        )}
      </div>
    </>
  );
}
