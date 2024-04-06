import { ChangeEvent, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import moment from 'moment';
import { motion } from 'framer-motion';
import { HexString } from '@gear-js/api';
import { useAccount, withoutCommas } from '@gear-js/react-hooks';
import { Button, Dropdown, Search } from '@ui';
import { cx } from '@/utils';
import { StreamTeaser } from '../StreamTeaser/StreamTeaser';
import styles from './StreamTeasersList.module.scss';
import { selectTeasersMenuAll, selectTeasersMenuAuthorized } from '../../config';
import { FormattedTeaser } from '../../types';
import { StreamTeasersListProps } from './StreamTeasersList.interfaces';
import { useProgramState } from '@/hooks';

function StreamTeasersList({ initialTeasersCount = 6, streamTeasersToExpand = 3 }: StreamTeasersListProps) {
  const { account } = useAccount();
  const {
    state: { streamTeasers, users },
  } = useProgramState();
  const selectTeasersMenu = users?.[account?.decodedAddress as HexString]
    ? { ...selectTeasersMenuAll, ...selectTeasersMenuAuthorized }
    : selectTeasersMenuAll;
  const [teasers, setTeasers] = useState<FormattedTeaser[]>([]);
  const [showedTeasersCount, setShowedTeasersCount] = useState<number>(initialTeasersCount);
  const [selectedStreamsOption, setSelectedStreamsOption] = useState<string>(selectTeasersMenu.all.label);
  const [searchedValue, setSearchedValue] = useState<string>('');
  const [showedTeasers, setShowedTeasers] = useState<FormattedTeaser[]>([]);

  useEffect(() => {
    if (streamTeasers && Object.keys(streamTeasers).length) {
      setTeasers(
        Object.keys(streamTeasers)
          .map((key) => ({ ...streamTeasers[key], id: key }))
          .sort((a, b) => {
            const aTimeCreation = moment(Number(a.timeCreation.replace(/,/g, '')));
            const bTimeCreation = moment(Number(b.timeCreation.replace(/,/g, '')));

            return bTimeCreation.diff(aTimeCreation);
          }),
      );
    }
  }, [streamTeasers]);

  const handleExpandPage = () => {
    setShowedTeasersCount((prev) => prev + streamTeasersToExpand);
  };

  const handleChangedSearchedValue = (e: ChangeEvent<HTMLInputElement>) => {
    setSearchedValue(e.target.value);
  };

  useEffect(() => {
    setShowedTeasers(teasers);
    setShowedTeasersCount(initialTeasersCount);
  }, [teasers, initialTeasersCount]);

  const handleSelectTypeOfStreams = ({ value, label }: (typeof selectTeasersMenu)[keyof typeof selectTeasersMenu]) => {
    setSearchedValue('');
    setShowedTeasersCount(initialTeasersCount);
    setSelectedStreamsOption(label);

    if (value === 'subscription') {
      const foundTeasers = teasers.filter((teaser) =>
        users?.[teaser.broadcaster].subscribers.includes(account?.decodedAddress || ''),
      );
      setShowedTeasers(foundTeasers);

      return;
    }

    if (value === 'upcoming') {
      const foundStreams = teasers
        .filter((teaser) => moment.unix(Number(withoutCommas(teaser.startTime)) / 1000).valueOf() > moment().valueOf())
        .sort((a, b) =>
          moment.unix(Number(withoutCommas(a.startTime)) / 1000).valueOf() >
          moment.unix(Number(withoutCommas(b.startTime)) / 1000).valueOf()
            ? 1
            : -1,
        );
      setShowedTeasers(foundStreams);

      return;
    }

    if (value === 'my') {
      const foundTeasers = teasers
        .filter((teaser) => teaser.broadcaster === account?.decodedAddress)
        .sort((a, b) =>
          moment.unix(Number(withoutCommas(a.startTime)) / 1000).valueOf() >
          moment.unix(Number(withoutCommas(b.startTime)) / 1000).valueOf()
            ? 1
            : -1,
        );
      setShowedTeasers(foundTeasers);

      return;
    }

    setShowedTeasers(teasers);
  };

  useEffect(() => {
    setShowedTeasers(teasers);
    setSearchedValue('');
    setSelectedStreamsOption(selectTeasersMenu.all.label);

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account?.decodedAddress]);

  const getSearchedTeasers = (items: FormattedTeaser[]) =>
    items.filter((teaser) => teaser.title.toLowerCase().includes(searchedValue));

  return (
    <div className={cx(styles.container)}>
      <div className={cx(styles.header)}>
        <Dropdown
          label={<h3 className={cx(styles['dropdown-title'])}>{selectedStreamsOption}</h3>}
          menu={selectTeasersMenu}
          activeValue={selectTeasersMenu.all.value}
          toggleArrowSize="medium"
          alignMenu="left"
          onItemClick={handleSelectTypeOfStreams}
        />
        <Search onChange={handleChangedSearchedValue} value={searchedValue} />
      </div>
      <div className={cx(styles.content)}>
        {showedTeasers.length > 0 ? (
          getSearchedTeasers(showedTeasers)
            .slice(0, showedTeasersCount)
            .map((item) => (
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
      {!getSearchedTeasers(showedTeasers).length && searchedValue ? (
        <h3 className={cx(styles['no-streams-found'])}>No streams found</h3>
      ) : null}
      {showedTeasersCount <= getSearchedTeasers(showedTeasers).length && (
        <div className={cx(styles['view-more-button-wrapper'])}>
          <Button variant="outline" size="medium" label="View More" onClick={handleExpandPage} />
        </div>
      )}
    </div>
  );
}

export { StreamTeasersList };
