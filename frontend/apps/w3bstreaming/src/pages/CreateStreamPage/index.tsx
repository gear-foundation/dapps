import { useAccount } from '@gear-js/react-hooks';
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { useGetStateQuery } from '@/app/utils';
import { CreateStreamRestrictModal } from '@/features/Auth/components';
import { LayoutCreateForm } from '@/features/CreateStream/components/LayoutCreateForm';

function CreateStreamPage() {
  const { account } = useAccount();
  const { users } = useGetStateQuery();
  const navigate = useNavigate();
  const [isModal, setIsModal] = useState<boolean>(false);

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

  return (
    <>
      <LayoutCreateForm />
      {isModal && <CreateStreamRestrictModal onClose={handleCloseModal} />}
    </>
  );
}

export { CreateStreamPage };
