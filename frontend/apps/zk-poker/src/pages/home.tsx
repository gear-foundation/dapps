import { Button as VaraButton } from '@gear-js/vara-ui';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { CreateGameIllustration, EditIcon, JoinGameIllustration, PointsIcon } from '@/assets/images';
import { Avatar, Button, Banner, Footer, MenuButton, Stats, Header, Balance } from '@/components';

import styles from './home.module.scss';

export default function Home() {
  const navigate = useNavigate();

  const userName = 'Player';

  const onEditProfile = () => {
    console.log('edit profile');
  };

  const stats = [
    { value: '52.12%', label: 'Your Winrate' },
    { value: 12, label: 'Total Hands Played' },
    { value: 0, label: 'Hands Played Today' },
  ];

  const claimFreePTS = () => {
    console.log('claim free PTS');
  };

  return (
    <>
      <Header>
        <Balance value="52,582" unit="PTS" SVG={PointsIcon} />
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
            subtitle="~15 VARA gas."
            onClick={() => {
              navigate(ROUTES.CREATE_GAME);
            }}
            illustration={CreateGameIllustration}
          />
        </div>

        <h3 className={styles.welcome}>
          Welcome, <Avatar size="sm" className={styles.avatar} /> {userName}{' '}
          <VaraButton color="transparent" icon={EditIcon} onClick={onEditProfile} />
        </h3>

        <Stats items={stats} />

        {/* TODO: move to separate feature */}
        <Button className={styles.claim} onClick={claimFreePTS}>
          Claim your free PTS
        </Button>

        <Footer />
      </div>
    </>
  );
}
