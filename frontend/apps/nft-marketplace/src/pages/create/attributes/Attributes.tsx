import { Button, Input } from '@gear-js/ui';
import { ReactComponent as MinusSVG } from 'assets/images/form/minus.svg';
import { FieldArrayWithId, UseFormRegister } from 'react-hook-form';
import styles from './Attributes.module.scss';

type Props = {
  register: UseFormRegister<any>;
  fields: FieldArrayWithId[];
  onRemoveButtonClick: (index: number) => void;
};

function Attributes({ register, fields, onRemoveButtonClick }: Props) {
  const getFields = () =>
    fields.map(({ id }, index) => (
      <div key={id} className={styles.field}>
        <div className={styles.inputs}>
          <Input
            label="Key"
            gap="1/3"
            className={styles.input}
            {...register(`attributes.${index}.key`, { required: true })}
          />

          <Input
            label="Value"
            gap="1/3"
            className={styles.input}
            {...register(`attributes.${index}.value`, { required: true })}
          />
        </div>

        {index !== 0 && (
          <Button
            icon={MinusSVG}
            color="transparent"
            onClick={() => onRemoveButtonClick(index)}
            className={styles.button}
          />
        )}
      </div>
    ));

  return <div>{getFields()}</div>;
}

export { Attributes };
