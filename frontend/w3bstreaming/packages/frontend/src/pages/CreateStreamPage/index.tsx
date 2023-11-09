import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router';
import { useAtomValue } from 'jotai';
import { useAccount } from '@gear-js/react-hooks';
import { CreateStreamRestrictModal } from '@/features/Auth/components';
import { LayoutCreateForm } from '@/features/CreateStream/components/LayoutCreateForm';
import { useGetStreamMetadata } from '@/features/CreateStream/hooks';
import { USERS_ATOM } from '@/atoms';
import { Loader } from '@/components';

function CreateStreamPage() {
  const { account } = useAccount();
  const users = useAtomValue(USERS_ATOM);
  const navigate = useNavigate();
  const [isModal, setIsModal] = useState<boolean>(false);
  const { meta, isMeta } = useGetStreamMetadata();

  const handleCloseModal = () => {
    setIsModal(false);
    navigate('/account');
  };

  useEffect(() => {
    if (users && account?.decodedAddress) {
      if (!users[account.decodedAddress]) {
        setIsModal(true);
      } else {
        setIsModal(false);
      }
    }
  }, [users, account?.decodedAddress]);

  return isMeta ? (
    <>
      <LayoutCreateForm meta={meta} />
      {isModal && <CreateStreamRestrictModal onClose={handleCloseModal} />}
    </>
  ) : (
    <Loader />
  );
}

export { CreateStreamPage };
