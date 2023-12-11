import { ChangeEvent, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import moment from 'moment';
import { motion } from 'framer-motion';
import { Button, Search } from '@ui';
import { cx } from '@/utils';
import { StreamTeaser } from '../StreamTeaser/StreamTeaser';
import styles from './StreamTeasersList.module.scss';
// import { selectTeasersMenu } from '../../config';
import { FormattedTeaser } from '../../types';
import { StreamTeasersListProps } from './StreamTeasersList.interfaces';
import { useProgramState } from '@/hooks';

function StreamTeasersList({ initialTeasersCount = 6, streamTeasersToExpand = 3 }: StreamTeasersListProps) {
  const {
    state: { streamTeasers, users },
  } = useProgramState();
  const [teasers, setTeasers] = useState<FormattedTeaser[]>([]);
  const [showedTeasersCount, setShowedTeasersCount] = useState<number>(initialTeasersCount);
  const [searchedValue, setSearchedValue] = useState<string>('');
  const [showedTeasers, setShowedTeasers] = useState<FormattedTeaser[]>([]);

  useEffect(() => {
    if (streamTeasers && Object.keys(streamTeasers).length) {
      setTeasers(
        Object.keys(streamTeasers)
          .map((key) => ({ ...streamTeasers[key], id: key }))
          .sort((a, b) => {
            const aStartTime = moment.unix(Number(a.startTime.replace(/,/g, '')));
            const bStartTime = moment.unix(Number(b.startTime.replace(/,/g, '')));

            // Сортировка по убыванию
            return bStartTime.diff(aStartTime);
          }),
      );
    }
  }, [streamTeasers]);

  const handleExpandPage = () => {
    setShowedTeasersCount((prev) => prev + streamTeasersToExpand);
  };

  const handleChangedSearchedValue = (e: ChangeEvent<HTMLInputElement>) => {
    setSearchedValue(e.target.value);
    const foundTeasers = teasers.filter((teaser) => teaser.title.toLowerCase().includes(e.target.value.toLowerCase()));

    setShowedTeasers(foundTeasers);
  };

  useEffect(() => {
    const foundTeasers = teasers.filter((teaser) => teaser.title.toLowerCase().includes(searchedValue.toLowerCase()));

    setShowedTeasers(foundTeasers);
    setShowedTeasersCount(initialTeasersCount);
  }, [searchedValue, teasers, initialTeasersCount]);

  // const handleSelectTypeOfStreams = ({ value }: (typeof selectTeasersMenu)[keyof typeof selectTeasersMenu]) => {
  //   console.log(value); //TODO connect the data
  // };

  return (
    <div className={cx(styles.container)}>
      <div className={cx(styles.header)}>
        {/* <Dropdown
          label={<h3 className={cx(styles['dropdown-title'])}>All streams</h3>}
          menu={selectTeasersMenu}
          activeValue={selectTeasersMenu.all.value}
          toggleArrowSize="medium"
          alignMenu="left"
          onItemClick={handleSelectTypeOfStreams}
        /> */}
        <h3 className={cx(styles['dropdown-title'])}>All streams</h3>
        <Search onChange={handleChangedSearchedValue} value={searchedValue} />
      </div>
      <div className={cx(styles.content)}>
        {showedTeasers.length > 0 ? (
          showedTeasers.slice(0, showedTeasersCount).map((item) => (
            <motion.div
              key={item.title + item.description + item.startTime + item.endTime}
              initial={{ opacity: 0 }}
              whileInView={{ opacity: 1 }}
              viewport={{ once: false }}>
              <Link to={`/stream/${item.id}`} key={item.title + item.description + item.startTime + item.endTime}>
                <StreamTeaser broadcasterInfo={users?.[item?.broadcaster]} {...item} />
              </Link>
            </motion.div>
          ))
        ) : (
          <div>No streams yet</div>
        )}
      </div>
      {!showedTeasers.length && searchedValue ? (
        <h3 className={cx(styles['no-streams-found'])}>No streams found</h3>
      ) : null}
      {showedTeasersCount <= showedTeasers.length && (
        <div className={cx(styles['view-more-button-wrapper'])}>
          <Button variant="outline" size="medium" label="View More" onClick={handleExpandPage} />
        </div>
      )}
    </div>
  );
}

export { StreamTeasersList };
