import styles from './CollectionPreview.module.scss';
import { cx } from '@/utils';
import { ReactComponent as IcImage } from '../../assets/images/ic-image-24.svg';
import { ReactComponent as Clocks } from '../../assets/images/watch-later-24px.svg';
import { CollectionPreviewProps } from './CollectionPreview.interfaces';
import { DescriptionItem } from '@/components';
import { Button } from '@/ui';
import { getNotMintedTokens, getTimeFormatFromStateDate } from '../../utils';

const sliceConfig = (length: number) => {
  if (length === 1) {
    return 1;
  }

  if (length > 1 && length < 7) {
    return 4;
  }

  return 9;
};

function CollectionPreview({ collection }: CollectionPreviewProps) {
  const {
    collection: { name },
    timeCreation,
    tokens,
  } = collection;
  const nftMaxCount = 10;

  return (
    <div className={cx(styles.card)}>
      <div className={cx(styles['image-wrapper'], styles[`image-wrapper-mosaic-${sliceConfig(tokens.length)}`])}>
        {tokens.slice(tokens.length - sliceConfig(tokens.length), tokens.length).map((item) => (
          <img key={item.id} src={item.medium} alt="nft" className={cx(styles.image)} />
        ))}
        {getNotMintedTokens(tokens).length > 0 && (
          <Button
            label="Available to Mint!"
            variant="primary"
            size="small"
            className={cx(styles['available-to-mint-button'])}
          />
        )}
      </div>
      <div className={cx(styles.content)}>
        <h4 className={cx(styles.title)}>{name}</h4>
        <div className={cx(styles.info)}>
          <DescriptionItem icon={<IcImage />} text={`${tokens.length}/${nftMaxCount}`} />
          <DescriptionItem icon={<Clocks />} text={`${getTimeFormatFromStateDate(timeCreation)}`} />
        </div>
      </div>
    </div>
  );
}

export { CollectionPreview };
