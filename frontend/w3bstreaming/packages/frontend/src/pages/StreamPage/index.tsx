import { useParams } from 'react-router';
import { useAccount } from '@gear-js/react-hooks';
import { Watch, Broadcast } from '@/features/Stream/components';
import { Layout } from '@/features/Stream/components/Layout';
import { socket } from '@/utils';
import { useProgramState } from '@/hooks';

function StreamPage() {
  const { account } = useAccount();
  const { id: streamId } = useParams();
  const {
    state: { users, streamTeasers },
  } = useProgramState();

  const streamTeaser = streamTeasers?.[streamId as string];

  return (
    <div>
      {streamTeaser && account && users && (
        <>
          <div>
            {account?.decodedAddress === streamTeaser.broadcaster ? (
              <Broadcast socket={socket} streamId={streamId as string} />
            ) : (
              <Watch socket={socket} streamId={streamId as string} />
            )}
          </div>
          <Layout
            isBroadcaster={account?.decodedAddress === streamTeaser.broadcaster}
            broadcasterId={streamTeaser.broadcaster}
            title={streamTeaser.title}
            description={streamTeaser.description}
            startTime={new Date(Number(streamTeaser?.startTime?.replace(/,/g, '')))}
            broadcasterInfo={users?.[streamTeaser.broadcaster]}
            isUserSubscribed={users?.[streamTeaser.broadcaster]?.subscribers?.includes(account?.decodedAddress)}
            streamId={streamId}
          />
        </>
      )}
    </div>
  );
}

export { StreamPage };
