import { useRef } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { BackIcon, PlusIcon, SearchIcon } from '@/assets/images';
import { Button, Input, Room } from '@/components';

import styles from './rooms.module.scss';

export default function Rooms() {
  const navigate = useNavigate();
  const searchRef = useRef<HTMLInputElement>(null);
  const rooms = [
    {
      name: 'Room 1',
      totalPlayers: 10,
      currentPlayers: 5,
      buyIn: 15000,
      adminName: 'John Doe',
      time: 60,
    },
    {
      name: 'Room 2',
      totalPlayers: 8,
      currentPlayers: 8,
      buyIn: 50000,
      adminName: 'John Doe',
      time: 120,
    },
  ];

  const privateRooms = [
    {
      name: 'Room 1',
      totalPlayers: 6,
      currentPlayers: 6,
      buyIn: 15000,
      adminName: 'John Doe',
      time: 60,
    },
  ];

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
          {rooms.map((room) => (
            <Room key={room.name} {...room} />
          ))}

          {privateRooms.length > 0 && (
            <>
              <h2 className={styles.title}>Private rooms</h2>
              {privateRooms.map((room) => (
                <Room key={room.name} {...room} />
              ))}
            </>
          )}
        </div>
      </div>
    </>
  );
}
