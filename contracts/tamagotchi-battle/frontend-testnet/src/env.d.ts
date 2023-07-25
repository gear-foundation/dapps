interface ImportMetaEnv {
  readonly VITE_NODE_ADDRESS: string
  readonly VITE_GAME_ADDRESS: string
  readonly VITE_FT_ADDRESS: string
  readonly VITE_LEADERBOARD_ADDRESS: string
  readonly VITE_AUTH_API_ADDRESS: string
  readonly VITE_TESTNET_WEBSITE_ADDRESS: string
  readonly VITE_BATTLE_ADDRESS: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
