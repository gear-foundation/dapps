import { Button } from '@gear-js/vara-ui';
import clsx from 'clsx';
import { useAtom, useAtomValue, useSetAtom } from 'jotai';
import { RefObject, useEffect, useState } from 'react';

import { Switcher } from '@dapps-frontend/ui';

import { BattleState, Player } from '@/app/utils';
import { Segmented, Text } from '@/components';
import { UserSkullIcon, UserSmileIcon } from '@/features/game/assets/images';
import { BattleHistoryCard, BattleCard, PlayersList, List } from '@/features/game/components';
import { battleHistoryAtom, currentPlayersAtom, otherPairBattleWatchAtom } from '@/features/game/store';
import { PlayerStatus } from '@/features/game/types';

import styles from './battle-tabs.module.scss';

type Tabs = 'players' | 'history';

type BattleTabsProps = {
  battleState: BattleState;
  participantsMap: Record<string, Player>;
  isAlive: boolean;
  tabsRef: RefObject<HTMLDivElement>;
  isSpectating: boolean;
};

export const BattleTabs = ({ battleState, participantsMap, isAlive, tabsRef, isSpectating }: BattleTabsProps) => {
  const { participants, defeated_participants, battle_name, state } = battleState;
  const [selectedTab, setSelectedTab] = useState<Tabs>('players');
  const [showCurrentBattle, setShowCurrentBattle] = useState(true);

  const [battleHistory, setBattleHistory] = useAtom(battleHistoryAtom);
  const currentPlayers = useAtomValue(currentPlayersAtom);
  const setOtherPairBattleWatch = useSetAtom(otherPairBattleWatchAtom);

  const isTournamentOver = 'gameIsOver' in state;

  useEffect(() => {
    if (!isAlive && !isTournamentOver) {
      setSelectedTab('history');
      setShowCurrentBattle(false);
    }
  }, [isAlive, isTournamentOver]);

  const alivePlayersListItems = participants.map(([address, { user_name }]) => ({
    name: user_name,
    status: 'alive' as PlayerStatus,
    address,
  }));
  const defeatedPlayersListItems = defeated_participants.map(([address, { user_name }]) => ({
    name: user_name,
    status: 'defeated' as PlayerStatus,
    address,
  }));

  const playersListItems = [...alivePlayersListItems, ...defeatedPlayersListItems];

  const segmentedOptions = [
    {
      label: (
        <div className={styles.players}>
          <span>Players:</span>
          <div>
            {participants.length} <UserSmileIcon />
          </div>
          <div>
            {defeated_participants.length} <UserSkullIcon />
          </div>
        </div>
      ),
      value: 'players',
    },
    {
      label: 'Battle History',
      value: 'history',
    },
  ];

  const renderCurrentBattleItems = () => {
    if (!currentPlayers || !battleHistory) return [];

    return battleHistory.map((history, index) => {
      return (
        <div key={index} className={clsx(styles.historyItem, styles.disabled)}>
          <BattleHistoryCard
            {...currentPlayers.player.player_settings}
            {...history.player}
            name={currentPlayers.player.user_name}
          />

          <BattleHistoryCard
            {...currentPlayers.opponent.player_settings}
            {...history.opponent}
            name={currentPlayers.opponent.user_name}
            align="right"
          />
        </div>
      );
    });
  };

  const renderBattleItems = () => {
    if (!battleState.pairs.length)
      return [
        <div key="empty">
          <Text>There are no other battles now</Text>
        </div>,
      ];

    return battleState.pairs.map(([key, { player_1, player_2 }]) => {
      const player1 = participantsMap[player_1];
      const player2 = participantsMap[player_2];
      const disabled = isAlive || !player2;

      const handleClick = () => {
        if (disabled) return;

        setOtherPairBattleWatch(key);

        setBattleHistory([
          {
            player: { action: null, health: player1.player_settings.health, isDodged: false, receivedDamage: 0 },
            opponent: { action: null, health: player1.player_settings.health, isDodged: false, receivedDamage: 0 },
          },
        ]);
      };

      return (
        <button
          type="button"
          key={key}
          className={clsx(styles.historyItem, disabled && styles.disabled)}
          onClick={handleClick}>
          <BattleCard
            {...player1.player_settings}
            name={player1.user_name}
            characterView={player1.appearance}
            winsCount={player1.number_of_victories}
          />

          {player2 && (
            <BattleCard
              {...player2.player_settings}
              name={player2.user_name}
              characterView={player2.appearance}
              winsCount={player2.number_of_victories}
              align="right"
            />
          )}
        </button>
      );
    });
  };

  return (
    <div className={clsx(styles.tabs, !isAlive && styles.defeated, isSpectating && styles.spectating)} ref={tabsRef}>
      {isSpectating ? (
        <>
          <Button
            text="Back to battles list"
            color="dark"
            className={styles.backButton}
            onClick={() => setOtherPairBattleWatch(null)}
            block
          />

          <List className={styles.list} maxLength={6} items={renderCurrentBattleItems()} />
        </>
      ) : (
        <>
          <Segmented
            options={segmentedOptions}
            value={selectedTab}
            onChange={(value) => setSelectedTab(value as Tabs)}
          />

          {selectedTab === 'players' && (
            <PlayersList
              bid={Number(battleState.bid)}
              items={playersListItems}
              className={styles.playersList}
              tournamentName={battle_name}
            />
          )}

          {selectedTab === 'history' && (
            <>
              <div className={styles.switcher}>
                <Switcher
                  size="small"
                  checked={showCurrentBattle}
                  onChange={(isChecked) => setShowCurrentBattle(isChecked)}
                />

                <Text size="sm">Show current battle</Text>
              </div>

              {showCurrentBattle ? (
                <List className={styles.list} maxLength={6} items={renderCurrentBattleItems()} />
              ) : (
                <List className={styles.list} maxLength={7} items={renderBattleItems()} />
              )}
            </>
          )}
        </>
      )}
    </div>
  );
};
