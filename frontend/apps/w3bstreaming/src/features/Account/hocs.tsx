import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import moment from 'moment';
import { FC, useEffect, useState } from 'react';

import { useGetStateQuery } from '@/app/utils';
import { Loader } from '@/components';

import { SubscribersData, SubscriptionsData, UsersTableProps } from './components/UsersTable/UsersTable.interfaces';
import { WithDataProps } from './types';

function withData(
  Component: FC<UsersTableProps>,
  type: 'subscriptions' | 'subscribers',
): (props: WithDataProps) => JSX.Element {
  return function Wrapped({ name, ...props }: WithDataProps) {
    const { account } = useAccount();
    const { users } = useGetStateQuery();
    const [data, setData] = useState<SubscribersData[] | SubscriptionsData[]>([]);
    const [some, setSome] = useState<boolean>(true);

    useEffect(() => {
      setSome(true);
      if (account && users) {
        if (type === 'subscriptions') {
          const subscriptionsData =
            users[account.decodedAddress]?.subscriptions?.map((user) => ({
              id: user.account_id,
              Streamer: `${users[user.account_id].name} ${users[user.account_id].surname}`,
              img: users[user.account_id].img_link,
              'Date of next write-off': 'N/A',
              'Subscription Date': moment(Number(user.sub_date)).format('M.D.YYYY'),
            })) || [];
          setData(subscriptionsData);
        }
        if (type === 'subscribers') {
          const subcribersData =
            users[account.decodedAddress]?.subscribers?.map((id: HexString) => {
              const subscriber = users[id];
              const subscribtion = subscriber.subscriptions.find(
                (subscription) => subscription.account_id === account.decodedAddress,
              );

              return {
                id,
                img: subscriber.img_link,
                User: subscriber.name || subscriber.surname ? `${subscriber.name} ${subscriber.surname}` : id,
                'Subscription Date': moment(Number(subscribtion?.sub_date || '')).format('M.D.YYYY'),
                'Date of next write-off': 'N/A',
                'Last payment date': '\u2013',
              };
            }) || [];
          setData(subcribersData);
        }
      }
      setSome(false);
    }, [account, users]);

    return <>{some ? <Loader /> : <Component {...props} data={data} name={type} />}</>;
  };
}

export { withData };
