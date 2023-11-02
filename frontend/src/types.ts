interface SignInResponse {
    id: string;
    username: string;
    email: string;
    steam_id?: string;
    psn_id?: string;
    xbox_id?: string;
  }

  interface CustomError {
    error: string,
  }