import { useSendMessage } from '@gear-js/react-hooks';
import { useDnsProgramIds } from '@dapps-frontend/hooks';
import metaTxt from '@/assets/meta/w3bstreaming.meta.txt';
import { useGetStreamMetadata } from '../CreateStream/hooks';
import { useProgramMetadata } from '@/hooks';

function useEditProfileMessage() {
  const { meta } = useGetStreamMetadata();
  const { programId } = useDnsProgramIds();

  return useSendMessage(programId, meta);
}

function useCreateStreamMetadata() {
  return useProgramMetadata(metaTxt);
}

function useSubscribeToStreamMessage() {
  const meta = useCreateStreamMetadata();
  const { programId } = useDnsProgramIds();

  return useSendMessage(programId, meta);
}

export { useSubscribeToStreamMessage, useEditProfileMessage };
