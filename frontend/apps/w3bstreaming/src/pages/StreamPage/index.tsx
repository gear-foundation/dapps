import { useAccount } from '@gear-js/react-hooks';
import { useParams } from 'react-router-dom';

import { useGetStateQuery } from '@/app/utils';
import { Watch, Broadcast } from '@/features/Stream/components';
import { Layout } from '@/features/Stream/components/Layout';
import { socket } from '@/utils';

function StreamPage() {
  const { account } = useAccount();
  const { streams, users } = useGetStateQuery();
  const { id: streamId } = useParams();

  const streamTeaser = streams?.[streamId as string];

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
            startTime={new Date(Number(streamTeaser?.start_time))}
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
