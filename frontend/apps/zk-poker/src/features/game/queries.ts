import { gql } from 'urql';

export type Lobby = {
  address: string;
  currentPlayers: { address: string }[];
};

export const GetLobbiesQuery = gql<{
  lobbies: Lobby[];
}>`
  query GetLobbies {
    lobbies(where: { status_not_eq: "killed" }) {
      address
      currentPlayers {
        address
      }
    }
  }
`;

export const GetPlayerByIdQuery = gql<{
  playerById: {
    gamesToday: number;
    wins: number;
    games: number;
  };
}>`
  query ($id: String!) {
    playerById(id: $id) {
      gamesToday
      wins
      games
    }
  }
`;
