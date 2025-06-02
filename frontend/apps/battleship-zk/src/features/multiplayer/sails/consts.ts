const SERVICE_NAME = 'multiple';

const EVENT_NAME = {
  SUBSCRIBE_TO_JOINED_THE_GAME_EVENT: 'subscribeToJoinedTheGameEvent',
  SUBSCRIBE_TO_PLACEMENT_VERIFIED_EVENT: 'subscribeToPlacementVerifiedEvent',
  SUBSCRIBE_TO_MOVE_MADE_EVENT: 'subscribeToMoveMadeEvent',
  SUBSCRIBE_TO_END_GAME_EVENT: 'subscribeToEndGameEvent',
  SUBSCRIBE_TO_GAME_CANCELED_EVENT: 'subscribeToGameCanceledEvent',
  SUBSCRIBE_TO_GAME_LEFT_EVENT: 'subscribeToGameLeftEvent',
  SUBSCRIBE_TO_PLAYER_DELETED_EVENT: 'subscribeToPlayerDeletedEvent',
} as const;

export { SERVICE_NAME, EVENT_NAME };
