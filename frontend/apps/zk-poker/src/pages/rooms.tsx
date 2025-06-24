import { useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { useQuery } from 'urql';

import { ROUTES } from '@/app/consts';
import { BackIcon, PlusIcon, SearchIcon } from '@/assets/images';
import { Button, Input, Room } from '@/components';
import { GetLobbiesQuery, Lobby } from '@/features/game/queries';
import { useLobbiesQuery } from '@/features/game/sails';

import styles from './rooms.module.scss';

export default function Rooms() {
  const navigate = useNavigate();
  const searchRef = useRef<HTMLInputElement>(null);
  const { lobbies } = useLobbiesQuery();

  const [lobbiesData] = useQuery({
    query: GetLobbiesQuery,
  });

  const lobbiesMap = lobbiesData?.data?.lobbies.reduce(
    (acc, lobby) => {
      acc[lobby.address] = lobby;
      return acc;
    },
    {} as Record<string, Lobby>,
  );

  const sortedLobbies = lobbies?.sort((a, b) => {
    const nameA = a[1].lobby_name;
    const nameB = b[1].lobby_name;
    return nameA.localeCompare(nameB);
  });

  const handleSearch = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    searchRef.current?.focus();
  };

  return (
    <>
      <div className={styles.container}>
        <div className={styles.header}>
          <Button color="contrast" rounded className={styles.backButton} onClick={() => navigate(ROUTES.HOME)}>
            <BackIcon />
          </Button>
          <h1 className={styles.title}>Open rooms</h1>
          <Button color="contrast" rounded className={styles.addButton} onClick={() => navigate(ROUTES.CREATE_GAME)}>
            <PlusIcon />
          </Button>
        </div>

        <form className={styles.searchContainer} onSubmit={handleSearch}>
          <Input ref={searchRef} placeholder="Search for lobby" className={styles.search} />
          <Button color="transparent" className={styles.searchButton} type="submit">
            <SearchIcon />
          </Button>
        </form>

        <div className={styles.rooms}>
          {sortedLobbies?.map(([address, { admin_name, admin_id, lobby_name, starting_bank, time_per_move_ms }]) => (
            <Room
              key={address}
              name={lobby_name}
              adminName={admin_name}
              adminId={admin_id}
              currentPlayers={lobbiesMap?.[address]?.currentPlayers.length || 1}
              buyIn={Number(starting_bank)}
              time={Number(time_per_move_ms) / 1000}
              id={address}
            />
          ))}
        </div>
      </div>
    </>
  );
}
