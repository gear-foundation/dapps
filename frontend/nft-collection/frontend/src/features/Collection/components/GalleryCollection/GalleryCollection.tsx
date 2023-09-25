import { GalleryCollectionProps } from './GalleryCollection.interfaces';
import styles from './GalleryCollection.module.scss';
import { cx } from '@/utils';
import 'swiper/css';
import 'swiper/css/navigation';
import { Gallery } from '@/components/Gallery';
import { MultiSwitch } from '@/components/MultiSwitch/MultiSwitch';
import { Dropdown } from '@/ui';
import { Option } from '@/components/MultiSwitch/MultiSwitch.interfaces';

function GalleryCollection({ title, data, emptyText, switchMenu, filterOptions }: GalleryCollectionProps) {
  const handleItemClick = (option: Option) => {
    switchMenu?.find(({ name }) => option.name === name)?.onSelect?.();
  };

  const handleFilterItemClick = (key: string) => {
    if (filterOptions) {
      filterOptions[key]?.onSelect?.();
    }
  };

  return (
    <div className={cx(styles.container)}>
      <div className={cx(styles.header)}>
        <div className={cx(styles.heading)}>
          {title && <h4 className={cx(styles.title)}>{title}</h4>}
          <div className={cx(styles.preferences)}>
            {switchMenu && (
              <MultiSwitch
                options={switchMenu}
                defaultSelected={switchMenu.find((item) => item.activeByDefault)?.name}
                onSelectOption={(option) => handleItemClick(option)}
              />
            )}
            {filterOptions && (
              <Dropdown
                label="Available to Mint"
                menu={filterOptions}
                onItemClick={(key) => handleFilterItemClick(key)}
              />
            )}
          </div>
        </div>
        <span className={cx(styles.results)}>{data.length} results</span>
      </div>
      <div className={cx(styles['gallery-wrapper'])}>
        <Gallery data={data} emptyText={emptyText} />
      </div>
    </div>
  );
}

export { GalleryCollection };
