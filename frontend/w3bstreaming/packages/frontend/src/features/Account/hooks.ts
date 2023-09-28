import { useSendMessage } from '@gear-js/react-hooks';
import metaTxt from '@/assets/meta/meta.txt';
import { ADDRESS } from '@/consts';
import { useMetadata } from '@/hooks';

function useEditProfileMetadata() {
  return useMetadata(metaTxt);
}

function useEditProfileMessage() {
  const meta = useEditProfileMetadata();

  return useSendMessage(ADDRESS.CONTRACT, meta);
}

function useCreateStreamMetadata() {
  return useMetadata(metaTxt);
}

function useSubscribeToStreamMessage() {
  const meta = useCreateStreamMetadata();

  return useSendMessage(ADDRESS.CONTRACT, meta);
}

export { useSubscribeToStreamMessage, useEditProfileMessage };
