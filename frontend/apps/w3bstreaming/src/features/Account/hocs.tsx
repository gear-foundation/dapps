import moment from 'moment';
import { FC, useEffect, useState } from 'react';
import { useAccount, withoutCommas } from '@gear-js/react-hooks';
import { SubscribersData, SubscriptionsData, UsersTableProps } from './components/UsersTable/UsersTable.interfaces';
import { WithDataProps } from './types';
import { Loader } from '@/components';
import { useProgramState } from '@/hooks';

function withData(
  Component: FC<UsersTableProps>,
  type: 'subscriptions' | 'subscribers',
): (props: WithDataProps) => JSX.Element {
  return function Wrapped({ name, ...props }: WithDataProps) {
    const { account } = useAccount();
    const {
      state: { users },
    } = useProgramState();
    const [data, setData] = useState<SubscribersData[] | SubscriptionsData[]>([]);
    const [some, setSome] = useState<boolean>(true);

    useEffect(() => {
      setSome(true);
      if (account && users) {
        if (type === 'subscriptions') {
          const subscriptionsData =
            users[account.decodedAddress]?.subscriptions?.map((user) => ({
              id: user.accountId,
              Streamer: `${users[user.accountId].name} ${users[user.accountId].surname}`,
              img: users[user.accountId].imgLink,
              'Date of next write-off': user.nextWriteOff || 'N/A',
              'Subscription Date': moment(Number(withoutCommas(user.subDate))).format('M.D.YYYY'),
            })) || [];
          setData(subscriptionsData);
        }
        if (type === 'subscribers') {
          const subcribersData =
            users[account.decodedAddress]?.subscribers?.map((id: string) => {
              const subscriber = users[id];
              const subscribtion = subscriber.subscriptions.find(
                (subscription) => subscription.accountId === account.decodedAddress,
              );

              return {
                id,
                img: subscriber.imgLink,
                User: subscriber.name || subscriber.surname ? `${subscriber.name} ${subscriber.surname}` : id,
                'Subscription Date': moment(Number(withoutCommas(subscribtion?.subDate || ''))).format('M.D.YYYY'),
                'Date of next write-off': subscribtion?.nextWriteOff || 'N/A',
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
