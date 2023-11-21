export type SignInResponse = {
  accessToken: string;
  username: string;
};

export type ISignInError = {
  errors?: {
    message: string;
  };
  message?: string;
};
