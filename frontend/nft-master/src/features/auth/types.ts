type SignInResponse = {
  accessToken: string;
};

type AuthResponse = {
  email: string;
  id: string;
  publicKey: string;
};

export type { SignInResponse, AuthResponse };

export type ISignInError = {
  errors?: {
    message: string;
  };
  message?: string;
};
