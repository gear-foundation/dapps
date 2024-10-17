import { useState } from 'react';
import { useAtom, useSetAtom } from 'jotai';

import { BattleHistoryCard, BattleCard, PlayersList, List } from '@/features/game/components';
import { Segmented } from '@/components';
import { BattleState, Player } from '@/app/utils';
import { UserSkullIcon, UserSmileIcon } from '@/features/game/assets/images';
import { PlayerStatus } from '@/features/game/types';
import { battleHistoryAtom, otherPairBattleWatchAtom } from '@/features/game/store';
import styles from './battle-tabs.module.scss';

type Tabs = 'players' | 'history';

type BattleTabsProps = {
  battleState: BattleState;
  participantsMap: Record<string, Player>;
  player: Player | null;
  opponent: Player | null;
  isAlive: boolean;
};

export const BattleTabs = ({ battleState, participantsMap, player, opponent, isAlive }: BattleTabsProps) => {
  const { participants, defeated_participants, battle_name } = battleState;
  const [selectedTab, setSelectedTab] = useState<Tabs>('players');

  const [battleHistory, setBattleHistory] = useAtom(battleHistoryAtom);
  const setOtherPairBattleWatch = useSetAtom(otherPairBattleWatchAtom);

  const showOtherBattles = !isAlive;
  const showPlayersList = selectedTab === 'players';

  const alivePlayersListItems = participants.map(([_, { user_name }]) => ({
    name: user_name,
    status: 'alive' as PlayerStatus,
  }));
  const defeatedPlayersListItems = defeated_participants.map(([_, { user_name }]) => ({
    name: user_name,
    status: 'defeated' as PlayerStatus,
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
      label: 'Battle History ',
      value: 'history',
    },
  ];
  return (
    <>
      <Segmented
        className={styles.segmented}
        options={segmentedOptions}
        value={selectedTab}
        onChange={(value) => setSelectedTab(value as Tabs)}
      />

      {showPlayersList && (
        <PlayersList
          bid={Number(battleState.bid)}
          items={playersListItems}
          className={styles.list}
          tournamentName={battle_name}
        />
      )}

      {selectedTab === 'history' && (
        <>
          {showOtherBattles ? (
            <List
              className={styles.list}
              maxLength={7}
              items={battleState.pairs.map(([key, { player_1, player_2 }]) => {
                const player1 = participantsMap[player_1];
                const player2 = participantsMap[player_2];

                return (
                  <div
                    key={key}
                    className={styles.historyItem}
                    onClick={() => {
                      setOtherPairBattleWatch(key);
                      setBattleHistory([
                        {
                          player: {
                            action: 'Attack',
                            health: player1.player_settings.health,
                            isDodged: false,
                            receivedDamage: 0,
                          },
                          opponent: {
                            action: 'Attack',
                            health: player1.player_settings.health,
                            isDodged: false,
                            receivedDamage: 0,
                          },
                        },
                      ]);
                    }}>
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
                  </div>
                );
              })}
            />
          ) : (
            <List
              className={styles.list}
              maxLength={6}
              items={
                (opponent &&
                  player &&
                  battleHistory?.map((history, index) => {
                    return (
                      <div key={index} className={styles.historyItem}>
                        <BattleHistoryCard {...player.player_settings} {...history.player} name={player.user_name} />
                        <BattleHistoryCard
                          {...opponent.player_settings}
                          {...history.opponent}
                          name={opponent.user_name}
                          align="right"
                        />
                      </div>
                    );
                  })) ||
                []
              }
            />
          )}
        </>
      )}
    </>
  );
};
