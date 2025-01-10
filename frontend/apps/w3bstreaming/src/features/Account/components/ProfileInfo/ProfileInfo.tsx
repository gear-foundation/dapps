import { useEffect, useState } from 'react';
import moment from 'moment-timezone';
import { useAccount } from '@gear-js/react-hooks';
import { useForm, isNotEmpty } from '@mantine/form';
import styles from './ProfileInfo.module.scss';
import { FormValues } from './ProfileInfo.interfaces';
import { cx } from '@/utils';
import { Button, TextField } from '@/ui';
import EditProfileIcon from '@/assets/icons/edit-profile-icon.svg';
import SuccessIcon from '@/assets/icons/success-icon.svg';
import CrossIcon from '@/assets/icons/cross-circle-icon.svg';
import defaultUserImg from '@/assets/icons/no-avatar-user-img.png';
import { PictureDropzone } from '@/features/CreateStream/components/PictureDropzone';
import picImage from '@/assets/icons/picture.png';
import { Select } from '@/ui/Select';
import { Profile, useEditProfileMessage, useGetStateQuery } from '@/app/utils';
import { usePending } from '@/app/hooks';

function ProfileInfo() {
  const { account } = useAccount();
  const { users, refetch } = useGetStateQuery();

  const { editProfileMessage } = useEditProfileMessage();
  const [userInfo, setUserInfo] = useState<Profile | null>(null);
  const [isEditingProfile, setIsEditingProfile] = useState<boolean>(false);
  const { pending } = usePending();

  const validateName = (value: string, name: string) => {
    if (value.length > 16) {
      return `${name} must be less than 16 symbols`;
    }

    if (!value.trim().length) {
      return `You must enter ${name}`;
    }

    return null;
  };

  const form = useForm<FormValues>({
    initialValues: {
      name: userInfo?.name || '',
      surname: userInfo?.surname || '',
      img_link: userInfo?.img_link || '',
      time_zone: userInfo?.time_zone || '',
    },
    validate: {
      name: (val) => validateName(val, 'Name'),
      surname: (val) => validateName(val, 'Surname'),
      time_zone: isNotEmpty('You must select your timezone'),
    },
  });

  const { getInputProps, setFieldValue, onSubmit, errors, reset } = form;

  const handleEditProfile = () => {
    setIsEditingProfile(true);

    setFieldValue('name', userInfo?.name || '');
    setFieldValue('surname', userInfo?.surname || '');
    setFieldValue('img_link', userInfo?.img_link || '');
    setFieldValue('time_zone', userInfo?.time_zone || '');
  };

  const handleCancelEditing = () => {
    setIsEditingProfile(false);
  };

  const handleDropImg = (prev: string[]) => {
    setFieldValue('img_link', prev[0]);
  };

  const handleSubmit = ({ name, surname, img_link, time_zone }: FormValues) => {
    console.log('submit');
    editProfileMessage(
      { name, surname, img_link, time_zone },
      {
        onSuccess: () => {
          setIsEditingProfile(false);
          setUserInfo((prev) => (prev ? { ...prev, name, surname, img_link, time_zone } : prev));
          reset();
          refetch();
        },
      },
    );
  };

  useEffect(() => {
    const user = users && account?.decodedAddress && users[account.decodedAddress];
    if (user) {
      setUserInfo(user);
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
          <img src={userInfo?.img_link || defaultUserImg} alt="profile" className={cx(styles['profile-info-image'])} />
          <p className={cx(styles['profile-info-name'])}>
            {userInfo?.name} {userInfo?.surname}
          </p>
        </>
      ) : (
        <form onSubmit={onSubmit(handleSubmit)}>
          <div className={cx(styles['dropzone-wrapper'])}>
            <PictureDropzone
              onDropFile={handleDropImg}
              previewLinks={getInputProps('img_link').value ? [getInputProps('img_link').value] : undefined}
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
              <TextField label="Enter name" disabled={pending} {...getInputProps('name')} />
              <span className={cx(styles['field-error'])}>{errors.name}</span>
            </div>
            <div className={cx(styles['form-item'])}>
              <TextField label="Enter surname" disabled={pending} {...getInputProps('surname')} />
              <span className={cx(styles['field-error'])}>{errors.surname}</span>
            </div>
            <div className={cx(styles['form-item'])}>
              <Select
                label="Timezone"
                disabled={pending}
                options={handleGetTimezones()}
                {...getInputProps('time_zone')}
              />
              <span className={cx(styles['field-error'])}>{errors.time_zone}</span>
            </div>
          </div>
          <div className={cx(styles.controls)}>
            <Button
              variant="primary"
              size="large"
              label="Save"
              icon={SuccessIcon}
              type="submit"
              isLoading={pending}
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
                disabled={pending}
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
          <p className={cx(styles['profile-info-role'])}>Speaker</p>
          {userInfo.time_zone && (
            <span className={cx(styles['profile-info-timezone'])}>
              {userInfo.time_zone.replace('/', ', ')}{' '}
              <span className={cx(styles.abbr)}>
                ({moment.tz(userInfo.time_zone).zoneAbbr()} {moment.tz(userInfo.time_zone).format('Z')})
              </span>
            </span>
          )}
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
