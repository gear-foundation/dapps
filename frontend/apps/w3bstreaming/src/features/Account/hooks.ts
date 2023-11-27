import { useSendMessage } from '@gear-js/react-hooks';
import metaTxt from '@/assets/meta/w3bstreaming.meta.txt';
import { ADDRESS } from '@/consts';
import { useGetStreamMetadata } from '../CreateStream/hooks';
import { useProgramMetadata } from '@/hooks';

function useEditProfileMessage() {
  const { meta } = useGetStreamMetadata();

  return useSendMessage(ADDRESS.CONTRACT, meta);
}

function useCreateStreamMetadata() {
  return useProgramMetadata(metaTxt);
}

function useSubscribeToStreamMessage() {
  const meta = useCreateStreamMetadata();

  return useSendMessage(ADDRESS.CONTRACT, meta);
}

export { useSubscribeToStreamMessage, useEditProfileMessage };
