import { Button, Checkbox, FileInput, Input, Textarea } from '@gear-js/ui';
import { ReactComponent as PlusSVG } from 'assets/images/form/plus.svg';
import { useIPFS, useNftMessage } from 'hooks';
import { getMintDetails, getMintPayload } from 'utils';
import { useForm, useFieldArray } from 'react-hook-form';
import { useEffect, useState } from 'react';
import { useAlert } from '@gear-js/react-hooks';
import { Attributes } from './attributes';
import styles from './Create.module.scss';

type AttributesValue = { key: string; value: string };
type Values = { name: string; description: string; attributes: AttributesValue[]; rarity: string };

const defaultAttributes = [{ key: '', value: '' }];
const defaultValues = { name: '', description: '', attributes: defaultAttributes, rarity: '' };

const IMAGE_FILE_TYPES = ['image/png', 'image/gif', 'image/jpeg'];

function Create() {
  const alert = useAlert();

  const { formState, control, register, handleSubmit, resetField, reset } = useForm<Values>({ defaultValues });
  const { fields, append, remove } = useFieldArray({ control, name: 'attributes' });
  const { errors } = formState;

  const ipfs = useIPFS();
  const sendMessage = useNftMessage();

  const [imageFile, setImageFile] = useState<File>();

  const [isAnyAttribute, setIsAnyAttribute] = useState(false);
  const [isRarity, setIsRarity] = useState(false);

  const handleImageFileChange = (value: File | undefined) => {
    if (!value) return setImageFile(value);
    if (value.size / 1024 ** 2 > 5) return alert.error('Image size should not exceed 5MB');
    if (!IMAGE_FILE_TYPES.includes(value.type)) return alert.error('Image should be .jpg, .png or .gif');

    setImageFile(value);
  };

  const toggleAttributes = () => setIsAnyAttribute((prevValue) => !prevValue);
  const toggleRarity = () => setIsRarity((prevValue) => !prevValue);

  useEffect(() => {
    resetField('attributes');
  }, [isAnyAttribute, resetField]);

  useEffect(() => {
    resetField('rarity');
  }, [isRarity, resetField]);

  const resetForm = () => {
    reset();
    setImageFile(undefined);
    setIsAnyAttribute(false);
    setIsRarity(false);
  };

  const onSubmit = async (data: Values) => {
    if (!imageFile) return alert.error('Image is required');

    const { name, description, attributes, rarity } = data;

    const details = isAnyAttribute || isRarity ? getMintDetails(isAnyAttribute ? attributes : undefined, rarity) : '';

    ipfs
      .add(imageFile)
      .then(({ cid }) => cid)
      .then(async (imageCid) => (details ? { detailsCid: (await ipfs.add(details)).cid, imageCid } : { imageCid }))
      .then(({ imageCid, detailsCid }) => getMintPayload(name, description, imageCid, detailsCid))
      .then((payload) => sendMessage({ payload, onSuccess: resetForm }));
  };

  return (
    <>
      <h2 className={styles.heading}>Create NFT</h2>

      <div className={styles.main}>
        <form className={styles.form} onSubmit={handleSubmit(onSubmit)}>
          <Input
            label="Name"
            gap="1/3"
            error={errors.name?.message}
            {...register('name', { required: 'Name is required' })}
          />

          <Textarea
            label="Description"
            gap="1/3"
            {...register('description', { required: 'Description is required' })}
            error={errors.description?.message}
          />

          <Checkbox
            label="Attributes"
            checked={isAnyAttribute}
            onChange={toggleAttributes}
            className={styles.checkbox}
          />

          {isAnyAttribute && <Button icon={PlusSVG} color="transparent" onClick={() => append(defaultAttributes)} />}
          {isAnyAttribute && <Attributes register={register} fields={fields} onRemoveButtonClick={remove} />}

          <Checkbox label="Rarity" checked={isRarity} onChange={toggleRarity} className={styles.checkbox} />

          {isRarity && (
            <Input
              label="Rarity"
              gap="1/3"
              error={errors.rarity?.message}
              {...register('rarity', { required: true })}
            />
          )}

          <FileInput
            label="Image"
            gap="1/3"
            accept={IMAGE_FILE_TYPES}
            value={imageFile}
            onChange={handleImageFileChange}
          />

          <Button type="submit" text="Create" block />
        </form>
      </div>
    </>
  );
}

export { Create };
