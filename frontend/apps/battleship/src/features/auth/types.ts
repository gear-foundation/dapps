export type SignInResponse = {
  accessToken: string;
  discord: string | null;
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
  email: string;
  id: string;
  publicKey: string;
  discord: string | null;
};

export type AvailableTokensResponse = {
  result: number;
};

export type IApiError = {
  code: number;
  content: {
    error: {}; // ??
    errors: {
      location: string; // "body"
      msg: string; // "Please enter valid email"
      path: string; // "email"
      type: string; // "field"
      value: string; // ""
    }[];
  };
  message: string;
};
