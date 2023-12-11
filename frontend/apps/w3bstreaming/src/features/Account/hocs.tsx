import { FC, useEffect, useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
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
              'Date of next write-off': user.nextWriteOff,
            })) || [];
          setData(subscriptionsData);
        }
        if (type === 'subscribers') {
          const subcribersData =
            users[account.decodedAddress]?.subscribers?.map((id: string) => ({
              id,
              img: users[id].imgLink,
              User: users[id].name || users[id].surname ? `${users[id].name} ${users[id].surname}` : id,
            })) || [];
          setData(subcribersData);
        }
      }
      setSome(false);
    }, [account, users]);

    return <>{some ? <Loader /> : <Component {...props} data={data} name={type} />}</>;
  };
}

export { withData };
