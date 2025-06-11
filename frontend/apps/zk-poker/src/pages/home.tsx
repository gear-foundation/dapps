import { useAccount } from '@gear-js/react-hooks';
import { Button as VaraButton } from '@gear-js/vara-ui';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { CreateGameIllustration, EditIcon, JoinGameIllustration, PointsIcon } from '@/assets/images';
import { Avatar, Banner, EditProfileModal, Footer, Header, MenuButton, Stats, Balance } from '@/components';
import { useUserName } from '@/features/game/hooks';
import { useGetBalanceQuery } from '@/features/game/sails';
import { ClaimPtsButton } from '@/features/pts';

import styles from './home.module.scss';

export default function Home() {
  const navigate = useNavigate();
  const { balance, refetch: refetchPtsBalance } = useGetBalanceQuery();
  const { userName, setUserName } = useUserName();
  const { account } = useAccount();

  const [isEditProfileModalOpen, setIsEditProfileModalOpen] = useState(false);

  const onEditProfile = () => {
    setIsEditProfileModalOpen(true);
  };

  const stats = [
    { value: '52.12%', label: 'Your Winrate' },
    { value: 12, label: 'Total Hands Played' },
    { value: 0, label: 'Hands Played Today' },
  ];

  const handleProfileSave = (name: string) => {
    setUserName(name);
    setIsEditProfileModalOpen(false);
  };

  const formattedBalance = balance?.toLocaleString('en-US') || '0';

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
            subtitle="12 rooms"
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

        <ClaimPtsButton onSuccess={refetchPtsBalance} className={styles.claim} />

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
