import { useRef } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { BackIcon, PlusIcon, SearchIcon } from '@/assets/images';
import { Button, Input, Room } from '@/components';
import { useLobbiesQuery } from '@/features/game/sails';

import styles from './rooms.module.scss';

export default function Rooms() {
  const navigate = useNavigate();
  const searchRef = useRef<HTMLInputElement>(null);
  const { lobbies } = useLobbiesQuery();

  const handleSearch = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    searchRef.current?.focus();
    console.log(searchRef.current?.value);
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
          {lobbies?.map(([address, { admin_name, admin_id, lobby_name, number_of_participants, starting_bank }]) => (
            <Room
              key={address}
              name={lobby_name}
              adminName={admin_name}
              adminId={admin_id}
              totalPlayers={number_of_participants}
              // ! TODO: get from indexer when it will be ready
              currentPlayers={1}
              buyIn={Number(starting_bank)}
              // ! TODO: add
              time={60}
              id={address}
            />
          ))}
        </div>
      </div>
    </>
  );
}
