export type SignInResponse = {
  accessToken: string;
  username: string;
};

export type AuthResponse = {
  email: string;
  id: string;
  publicKey: string;
  username: string;
};

export type ISignInError = {
  errors?: {
    message: string;
  };
  message?: string;
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
