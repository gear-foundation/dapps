export type SignInResponse = {
  accessToken: string;
  discord: string | null;
  username: string;
};

export type ShareLinkResponse = {
  link: string;
  registeredUserCount: number;
  remainingUsersToInvite: number;
};

export type LinkResponse = ShareLinkResponse & {
  expired?: boolean;
  freeze?: number;
  message?: string;
};

export type AuthResponse = {
  success: true;
  content: {
    user: {
      address: string;
      activities: {
        staked: boolean;
        raced: boolean;
        tictactoe: boolean;
      };
    };
  };
};

export type AvailableTokensResponse = {
  result: number;
};
