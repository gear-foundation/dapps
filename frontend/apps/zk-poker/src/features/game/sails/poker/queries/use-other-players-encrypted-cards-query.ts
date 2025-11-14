import { useAccount } from '@gear-js/react-hooks';
import { useQuery } from '@tanstack/react-query';

import { usePokerProgram } from '@/app/utils';

import { useParticipantsQuery } from './use-participants-query';

type Params = {
  enabled: boolean;
};

export const useOtherPlayersEncryptedCardsQuery = ({ enabled }: Params) => {
  const program = usePokerProgram();
  const { account } = useAccount();
  const { participants } = useParticipantsQuery();

  const { data, refetch, isFetching, error } = useQuery({
    queryKey: ['encryptedCards', account?.decodedAddress, participants],
    queryFn: async (): Promise<EncryptedCard[]> => {
      if (!program || !account || !participants) {
        return [];
      }

      // get all players except the current user
      const otherPlayers = participants
        .filter(([playerId]) => playerId !== account.decodedAddress)
        .map(([playerId]) => playerId);

      if (otherPlayers.length === 0) {
        return [];
      }

      const encryptedCardsPromises = otherPlayers.map((playerId) => program.poker.encryptedCards(playerId).call());

      const results = await Promise.all(encryptedCardsPromises);

      const flatCards = results.filter((cards): cards is EncryptedCard[] => cards !== null).flat();

      return flatCards;
    },
    enabled: !!account && !!program && !!participants && enabled,
    staleTime: 0, // Data always considered stale to get fresh cards in a new round
  });

  return { encryptedCards: data || null, isFetching, refetch, error };
};
