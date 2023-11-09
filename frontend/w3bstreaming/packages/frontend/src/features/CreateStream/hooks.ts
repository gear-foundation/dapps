import { useAtomValue, useSetAtom } from 'jotai';
import { useSendMessage } from '@gear-js/react-hooks';
import { ADDRESS } from '@/consts';
import { useMetadata } from '@/hooks';
import metaTxt from '@/assets/meta/meta.txt';
import { META_ATOM } from '@/atoms';

function useCreateStreamMetadata() {
  const setMeta = useSetAtom(META_ATOM);
  const metaData = useMetadata(metaTxt);

  setMeta(metaData);
}

function useGetStreamMetadata() {
  const meta = useAtomValue(META_ATOM);

  return { isMeta: !!meta, meta };
}

function useCreateStreamSendMessage() {
  const { meta } = useGetStreamMetadata();

  return useSendMessage(ADDRESS.CONTRACT, meta);
}

export { useCreateStreamSendMessage, useCreateStreamMetadata, useGetStreamMetadata };
