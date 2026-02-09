import { useQuery } from '@tanstack/react-query';

import { graphqlClient } from '@/app/utils';

export enum LobbyStatus {
  CREATED = 'created',
  STARTED = '{"registration":null}',
  FINISHED = 'finished',
  KILLED = 'killed',
}

export type Lobby = {
  address: string;
  status: string;
  currentPlayers: { address: string }[];
};

export type PlayerStats = {
  gamesToday: number;
  wins: number;
  games: number;
};

const GET_LOBBIES_QUERY = `
  query GetLobbies {
    lobbies(where: { status_not_eq: "killed" }) {
      address
      status
      currentPlayers {
        address
      }
    }
  }
`;

const GET_PLAYER_BY_ID_QUERY = `
  query GetPlayerById($id: String!) {
    playerById(id: $id) {
      gamesToday
      wins
      games
    }
  }
`;

export const useGetLobbiesQuery = () => {
  return useQuery({
    queryKey: ['lobbies'],
    queryFn: async (): Promise<{ lobbies: Lobby[] }> => {
      try {
        const data = await graphqlClient.request<{ lobbies: Lobby[] }>(GET_LOBBIES_QUERY);
        return data;
      } catch (error) {
        console.error('Error fetching lobbies:', error);
        return { lobbies: [] };
      }
    },
  });
};

export const useGetPlayerByIdQuery = (id?: string) => {
  return useQuery({
    queryKey: ['player', id],
    queryFn: async (): Promise<{ playerById: PlayerStats }> => {
      if (!id) throw new Error('Player ID is required');
      try {
        const data = await graphqlClient.request<{ playerById: PlayerStats }>(GET_PLAYER_BY_ID_QUERY, { id });
        return data;
      } catch (error) {
        console.error('Error fetching player stats:', error);
        return { playerById: { gamesToday: 0, wins: 0, games: 0 } };
      }
    },
    enabled: !!id,
  });
};
