import { useQuery, type UseQueryResult } from '@tanstack/react-query';

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
  createdAt: string;
  lastUpdatedAt: string;
  currentPlayers: { address: string }[];
  timeUntilStartMs?: number | string | null;
  lobbyTimeLimitMs?: number | string | null;
};

export type PlayerStats = {
  gamesToday: number;
  wins: number;
  games: number;
};

type LobbyById = {
  status: string;
};

type LobbyStatusByIdResponse = {
  lobbyById: LobbyById | null;
};

const GET_LOBBIES_QUERY = `
  query GetLobbies($createdAtGte: DateTime!) {
    lobbies(where: { status_not_eq: "killed", createdAt_gte: $createdAtGte }) {
      address
      status
      createdAt
      lastUpdatedAt
      timeUntilStartMs
      lobbyTimeLimitMs
      currentPlayers {
        address
      }
    }
  }
`;

const DEFAULT_LOBBY_TTL_MS = 24 * 60 * 60 * 1000;

const toNumberOrNull = (value: number | string | null | undefined) => {
  if (value === null || value === undefined) return null;
  const parsed = Number(value);

  return Number.isFinite(parsed) ? parsed : null;
};

const getLobbyTtlMs = (lobby: Lobby) => {
  const timeUntilStartMs = toNumberOrNull(lobby.timeUntilStartMs);
  const lobbyTimeLimitRawMs = toNumberOrNull(lobby.lobbyTimeLimitMs);
  const lobbyTimeLimitMs = lobbyTimeLimitRawMs === 0 ? DEFAULT_LOBBY_TTL_MS : lobbyTimeLimitRawMs;

  if (timeUntilStartMs !== null && lobbyTimeLimitMs !== null) {
    return timeUntilStartMs + lobbyTimeLimitMs;
  }
  if (lobbyTimeLimitMs !== null) {
    return lobbyTimeLimitMs;
  }

  return DEFAULT_LOBBY_TTL_MS;
};

const isLobbyAlive = (lobby: Lobby, nowMs: number) => {
  const createdAtMs = new Date(lobby.createdAt).getTime();
  if (Number.isNaN(createdAtMs)) return false;

  return nowMs - createdAtMs <= getLobbyTtlMs(lobby);
};

const GET_PLAYER_BY_ID_QUERY = `
  query GetPlayerById($id: String!) {
    playerById(id: $id) {
      gamesToday
      wins
      games
    }
  }
`;

const GET_LOBBY_STATUS_BY_ID_QUERY = `
  query GetLobbyStatusById($id: String!) {
    lobbyById(id: $id) {
      status
    }
  }
`;

export const useGetLobbiesQuery = () => {
  return useQuery({
    queryKey: ['lobbies'],
    queryFn: async (): Promise<{ lobbies: Lobby[] }> => {
      try {
        const createdAtGte = new Date(Date.now() - DEFAULT_LOBBY_TTL_MS).toISOString();
        const data = await graphqlClient.request<{ lobbies: Lobby[] }>(GET_LOBBIES_QUERY, { createdAtGte });
        const nowMs = Date.now();
        const aliveLobbies = data.lobbies.filter((lobby) => isLobbyAlive(lobby, nowMs));

        return { lobbies: aliveLobbies };
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

export const useGetLobbyStatusByIdQuery = (id?: string): UseQueryResult<LobbyStatusByIdResponse, Error> => {
  return useQuery<LobbyStatusByIdResponse, Error>({
    queryKey: ['lobby-status', id],
    queryFn: async (): Promise<LobbyStatusByIdResponse> => {
      if (!id) throw new Error('Lobby ID is required');
      try {
        const data = await graphqlClient.request<{ lobbyById: LobbyById | null }>(GET_LOBBY_STATUS_BY_ID_QUERY, { id });
        return data;
      } catch (error) {
        console.error('Error fetching lobby status:', error);
        return { lobbyById: null };
      }
    },
    enabled: !!id,
  });
};
