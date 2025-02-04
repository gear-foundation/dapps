import { Button, Input, Modal } from '@gear-js/ui';
import { ChangeEvent, FormEvent, useState } from 'react';
import { MIN_PRICE } from '@/consts';
import styles from '../index.module.scss';
import { useAlert } from '@gear-js/react-hooks';

type Props = {
  heading: string;
  close: () => void;
  onSubmit: (value: string, onSuccess: () => void) => void;
};

function PriceModal({ heading, close, onSubmit }: Props) {
  const [price, setPrice] = useState('');
  const alert = useAlert();
  const handlePriceChange = ({ target: { value } }: ChangeEvent<HTMLInputElement>) => setPrice(value);

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (Number(price) >= MIN_PRICE) {
      onSubmit(price, close);
    } else {
      alert.error(`Minimum price is ${MIN_PRICE}`);
    }
  };

  return (
    <Modal heading={heading} close={close}>
      <form className={styles.form} onSubmit={handleSubmit}>
        <Input type="number" value={price} onChange={handlePriceChange} min={MIN_PRICE} />
        <Button type="submit" text="OK" block />
      </form>
    </Modal>
  );
}

export { PriceModal };
