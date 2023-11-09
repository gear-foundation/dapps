import { useEffect, useState } from 'react';
import { useAtomValue } from 'jotai';
import { useAccount, useAlert, withoutCommas } from '@gear-js/react-hooks';
import { useForm, isNotEmpty } from '@mantine/form';
import styles from './ProfileInfo.module.scss';
import { FormValues } from './ProfileInfo.interfaces';
import { cx, logger } from '@/utils';
import { Button, DropzoneUploader, Input } from '@/ui';
import EditProfileIcon from '@/assets/icons/edit-profile-icon.svg';
import SuccessIcon from '@/assets/icons/success-icon.svg';
import CrossIcon from '@/assets/icons/cross-circle-icon.svg';
import defaultUserImg from '@/assets/icons/no-avatar-user-img.png';
import { useEditProfileMessage } from '../../hooks';
import { User } from '../../types';
import { USERS_ATOM } from '@/atoms';
import { useGetStreamMetadata } from '@/features/CreateStream/hooks';
import { useCheckBalance, useHandleCalculateGas } from '@/hooks';
import { ADDRESS } from '@/consts';

function ProfileInfo() {
  const { account } = useAccount();
  const alert = useAlert();
  const { meta } = useGetStreamMetadata();
  const users = useAtomValue(USERS_ATOM);
  const sendMessage = useEditProfileMessage();
  const [userInfo, setUserInfo] = useState<User | null>(null);
  const [isEditingProfile, setIsEditingProfile] = useState<boolean>(false);
  const calculateGas = useHandleCalculateGas(ADDRESS.CONTRACT, meta);
  const { checkBalance } = useCheckBalance();

  const form = useForm<FormValues>({
    initialValues: {
      name: userInfo?.name || '',
      surname: userInfo?.surname || '',
      imgLink: userInfo?.imgLink || '',
    },
    validate: {
      name: isNotEmpty('You must enter surname'),
      surname: isNotEmpty('You must enter surname'),
    },
  });

  const { getInputProps, setFieldValue, onSubmit, errors } = form;

  const handleEditProfile = () => {
    setIsEditingProfile(true);

    setFieldValue('name', userInfo?.name || '');
    setFieldValue('surname', userInfo?.surname || '');
    setFieldValue('imgLink', userInfo?.imgLink || '');
  };

  const handleCancelEditing = () => {
    setIsEditingProfile(false);
  };

  const handleDropImg = (prev: string) => {
    setFieldValue('imgLink', prev);
  };

  const handleSubmit = ({ name, surname, imgLink }: FormValues) => {
    const payload = {
      EditProfile: {
        name,
        surname,
        imgLink,
      },
    };

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const minLimit = withoutCommas(min_limit as string);
        const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2);
        logger(`Calculating gas:`);
        logger(`MIN_LIMIT ${min_limit}`);
        logger(`LIMIT ${gasLimit}`);
        logger(`Calculated gas SUCCESS`);
        logger(`Sending message`);

        checkBalance(
          gasLimit,
          () =>
            sendMessage({
              payload,
              gasLimit,
              onError: () => {
                logger(`Errror send message`);
              },
              onSuccess: (messageId) => {
                logger(`sucess on ID: ${messageId}`);
                if (isEditingProfile) {
                  setIsEditingProfile(false);
                }
                setUserInfo((prev) =>
                  prev
                    ? {
                        ...prev,
                        name,
                        surname,
                        imgLink,
                      }
                    : prev,
                );
              },
              onInBlock: (messageId) => {
                logger('messageInBlock');
                logger(`messageID: ${messageId}`);
              },
            }),
          () => {
            logger(`Errror check balance`);
          },
        );
      })
      .catch((error) => {
        logger(error);
        alert.error('Gas calculation error');
      });
  };

  useEffect(() => {
    if (users && account?.decodedAddress) {
      if (users[account.decodedAddress]) {
        setUserInfo(users[account.decodedAddress]);
      }
    }
  }, [users, account?.decodedAddress]);

  return (
    <div className={cx(styles['profile-info'])}>
      {userInfo && !isEditingProfile ? (
        <>
          <img src={userInfo?.imgLink || defaultUserImg} alt="profile" className={cx(styles['profile-info-image'])} />
          <p className={cx(styles['profile-info-name'])}>
            {userInfo?.name} {userInfo?.surname}
          </p>
        </>
      ) : (
        <form onSubmit={onSubmit(handleSubmit)}>
          <div className={cx(styles['dropzone-wrapper'])}>
            <DropzoneUploader text="" onDropFile={handleDropImg} previewLink={getInputProps('imgLink').value} />
          </div>
          <div className={cx(styles['profile-info-form-fields'])}>
            <div className={cx(styles['form-item'])}>
              <Input placeholder="Enter name" {...getInputProps('name')} />
              <span className={cx(styles['field-error'])}>{errors.name}</span>
            </div>
            <div className={cx(styles['form-item'])}>
              <Input placeholder="Enter surname" {...getInputProps('surname')} />
              <span className={cx(styles['field-error'])}>{errors.surname}</span>
            </div>
          </div>
          <div className={cx(styles.controls)}>
            <Button
              variant="primary"
              size="large"
              label="Save"
              icon={SuccessIcon}
              type="submit"
              className={cx(styles['save-button'])}
            />
            <Button
              variant="outline"
              size="large"
              label="Cancel"
              icon={CrossIcon}
              onClick={handleCancelEditing}
              className={cx(styles['save-button'])}
            />
          </div>
        </form>
      )}
      {userInfo && !isEditingProfile && (
        <>
          <p className={cx(styles['profile-info-subs'])}>
            <span className={cx(styles['profile-info-subs-value'])}>{userInfo.subscribers.length} </span>
            <span className={cx(styles['profile-info-subs-caption'])}>subscribers</span>
          </p>
          <p className={cx(styles['profile-info-role'])}>{userInfo?.role}</p>
          <Button
            variant="outline"
            size="large"
            label="Edit Profile"
            icon={EditProfileIcon}
            onClick={handleEditProfile}
          />
        </>
      )}
    </div>
  );
}

export { ProfileInfo };
