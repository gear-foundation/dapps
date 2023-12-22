import { useEffect, useState } from 'react';
import { useAtom } from 'jotai';
import moment from 'moment-timezone';
import { useAccount, useAlert, withoutCommas } from '@gear-js/react-hooks';
import { useForm, isNotEmpty } from '@mantine/form';
import styles from './ProfileInfo.module.scss';
import { FormValues } from './ProfileInfo.interfaces';
import { cx, logger } from '@/utils';
import { Button, TextField } from '@/ui';
import EditProfileIcon from '@/assets/icons/edit-profile-icon.svg';
import SuccessIcon from '@/assets/icons/success-icon.svg';
import CrossIcon from '@/assets/icons/cross-circle-icon.svg';
import defaultUserImg from '@/assets/icons/no-avatar-user-img.png';
import { useEditProfileMessage } from '../../hooks';
import { User } from '../../types';
import { useGetStreamMetadata } from '@/features/CreateStream/hooks';
import { useCheckBalance, useHandleCalculateGas, useProgramState } from '@/hooks';
import { ADDRESS } from '@/consts';
import { IS_CREATING_ACCOUNT_ATOM } from '../../atoms';
import { PictureDropzone } from '@/features/CreateStream/components/PictureDropzone';
import picImage from '@/assets/icons/picture.png';
import { Select } from '@/ui/Select';

function ProfileInfo() {
  const { account } = useAccount();
  const alert = useAlert();
  const { meta } = useGetStreamMetadata();
  const {
    state: { users },
    updateUsers,
  } = useProgramState();
  const sendMessage = useEditProfileMessage();
  const [userInfo, setUserInfo] = useState<User | null>(null);
  const [isEditingProfile, setIsEditingProfile] = useState<boolean>(false);
  const [isCreatingAccount, setIsCreatingAccount] = useAtom(IS_CREATING_ACCOUNT_ATOM);
  const calculateGas = useHandleCalculateGas(ADDRESS.CONTRACT, meta);
  const { checkBalance } = useCheckBalance();

  const form = useForm<FormValues>({
    initialValues: {
      name: userInfo?.name || '',
      surname: userInfo?.surname || '',
      imgLink: userInfo?.imgLink || '',
      timezone: userInfo?.timeZone || '',
    },
    validate: {
      name: isNotEmpty('You must enter name'),
      surname: isNotEmpty('You must enter surname'),
      timezone: isNotEmpty('You must select your timezone'),
    },
  });

  const { getInputProps, setFieldValue, onSubmit, errors, reset } = form;

  const handleEditProfile = () => {
    setIsEditingProfile(true);

    setFieldValue('name', userInfo?.name || '');
    setFieldValue('surname', userInfo?.surname || '');
    setFieldValue('imgLink', userInfo?.imgLink || '');
    setFieldValue('timezone', userInfo?.timeZone || '');
  };

  const handleCancelEditing = () => {
    setIsEditingProfile(false);
  };

  const handleDropImg = (prev: string[]) => {
    setFieldValue('imgLink', prev[0]);
  };

  const handleSubmit = ({ name, surname, imgLink, timezone }: FormValues) => {
    setIsCreatingAccount(true);
    const payload = {
      EditProfile: {
        name,
        surname,
        imgLink,
        timeZone: timezone,
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
                setIsCreatingAccount(false);
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
                        timezone,
                      }
                    : prev,
                );
                reset();
                updateUsers();
                setIsCreatingAccount(false);
              },
              onInBlock: (messageId) => {
                logger('messageInBlock');
                logger(`messageID: ${messageId}`);
              },
            }),
          () => {
            logger(`Errror check balance`);
            setIsCreatingAccount(false);
          },
        );
      })
      .catch((error) => {
        setIsCreatingAccount(false);
        logger(error);
        alert.error('Gas calculation error');
      });
  };

  useEffect(() => {
    if (users && account?.decodedAddress && users[account.decodedAddress]) {
      setUserInfo(users[account.decodedAddress]);
    } else {
      setUserInfo(null);
    }
    handleCancelEditing();
    reset();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [users, account?.decodedAddress]);

  const handleGetTimezones = () => {
    const zones = moment.tz.names();

    return zones.map((zone) => {
      if (zone.includes('/')) {
        const abbr = moment.tz(zone).zoneAbbr();
        const offset = moment.tz(zone).format('Z');

        return {
          label: `${zone} (${abbr} ${offset})`,
          value: zone,
        };
      }

      const abbr = moment.tz(zone).zoneAbbr();
      const offset = moment.tz(zone).format('Z');

      return {
        label: `${zone} (${abbr} ${offset})`,
        value: zone,
      };
    });
  };

  return (
    <div className={cx(styles['profile-info'])}>
      {!!userInfo && !isEditingProfile ? (
        <>
          <img src={userInfo?.imgLink || defaultUserImg} alt="profile" className={cx(styles['profile-info-image'])} />
          <p className={cx(styles['profile-info-name'])}>
            {userInfo?.name} {userInfo?.surname}
          </p>
        </>
      ) : (
        <form onSubmit={onSubmit(handleSubmit)}>
          <div className={cx(styles['dropzone-wrapper'])}>
            <PictureDropzone
              onDropFile={handleDropImg}
              previewLinks={getInputProps('imgLink').value ? [getInputProps('imgLink').value] : undefined}
              content={
                <div className={cx(styles.label)}>
                  <img src={picImage} alt="upload" />
                  <h5 className={cx(styles['label-title'])}>Upload photo</h5>
                </div>
              }
            />
          </div>
          <div className={cx(styles['profile-info-form-fields'])}>
            <div className={cx(styles['form-item'])}>
              <TextField label="Enter name" disabled={isCreatingAccount} {...getInputProps('name')} />
              <span className={cx(styles['field-error'])}>{errors.name}</span>
            </div>
            <div className={cx(styles['form-item'])}>
              <TextField label="Enter surname" disabled={isCreatingAccount} {...getInputProps('surname')} />
              <span className={cx(styles['field-error'])}>{errors.surname}</span>
            </div>
            <div className={cx(styles['form-item'])}>
              <Select
                label="Timezone"
                disabled={isCreatingAccount}
                options={handleGetTimezones()}
                {...getInputProps('timezone')}
              />
              <span className={cx(styles['field-error'])}>{errors.timezone}</span>
            </div>
          </div>
          <div className={cx(styles.controls)}>
            <Button
              variant="primary"
              size="large"
              label="Save"
              icon={SuccessIcon}
              type="submit"
              isLoading={isCreatingAccount}
              className={cx(styles['save-button'])}
            />
            {!!isEditingProfile && (
              <Button
                variant="outline"
                size="large"
                label="Cancel"
                icon={CrossIcon}
                onClick={handleCancelEditing}
                className={cx(styles['save-button'])}
                disabled={isCreatingAccount}
              />
            )}
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
          <span className={cx(styles['profile-info-timezone'])}>
            {userInfo.timeZone.replace('/', ', ')}{' '}
            <span className={cx(styles.abbr)}>
              ({moment.tz(userInfo.timeZone).zoneAbbr()} {moment.tz(userInfo.timeZone).format('Z')})
            </span>
          </span>
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
