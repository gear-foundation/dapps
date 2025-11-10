import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { motion } from 'framer-motion';
import moment from 'moment';
import { ChangeEvent, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';

import { useGetStateQuery } from '@/app/utils';
import { Button, Dropdown, Search, DropdownMenuItem } from '@/ui';
import { cx } from '@/utils';

import { selectTeasersMenuAll, selectTeasersMenuAuthorized } from '../../config';
import { FormattedTeaser } from '../../types';
import { StreamTeaser } from '../StreamTeaser/StreamTeaser';

import { StreamTeasersListProps } from './StreamTeasersList.interfaces';
import styles from './StreamTeasersList.module.scss';

function StreamTeasersList({ initialTeasersCount = 6, streamTeasersToExpand = 3 }: StreamTeasersListProps) {
  const { account } = useAccount();
  const { users, streams } = useGetStateQuery();
  const selectTeasersMenu = users?.[account?.decodedAddress as HexString]
    ? { ...selectTeasersMenuAll, ...selectTeasersMenuAuthorized }
    : selectTeasersMenuAll;
  const [teasers, setTeasers] = useState<FormattedTeaser[]>([]);
  const [showedTeasersCount, setShowedTeasersCount] = useState<number>(initialTeasersCount);
  const [selectedStreamsOption, setSelectedStreamsOption] = useState<string>(selectTeasersMenu.all.label);
  const [searchedValue, setSearchedValue] = useState<string>('');
  const [showedTeasers, setShowedTeasers] = useState<FormattedTeaser[]>([]);

  useEffect(() => {
    if (streams && Object.keys(streams).length) {
      setTeasers(
        Object.keys(streams)
          .map((key) => ({ ...streams[key], id: key }))
          .sort((a, b) => {
            const aTimeCreation = moment(Number(a.start_time));
            const bTimeCreation = moment(Number(b.start_time));

            return bTimeCreation.diff(aTimeCreation);
          }),
      );
    }
  }, [streams]);

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

  const handleSelectTypeOfStreams = ({ value, label }: DropdownMenuItem) => {
    setSearchedValue('');
    setShowedTeasersCount(initialTeasersCount);
    setSelectedStreamsOption(label);

    if (value === 'subscription') {
      const foundTeasers = teasers.filter(
        (teaser) => account && users?.[teaser.broadcaster].subscribers.includes(account.decodedAddress),
      );
      setShowedTeasers(foundTeasers);

      return;
    }

    if (value === 'upcoming') {
      const foundStreams = teasers
        .filter((teaser) => moment.unix(Number(teaser.start_time) / 1000).valueOf() > moment().valueOf())
        .sort((a, b) =>
          moment.unix(Number(a.start_time) / 1000).valueOf() > moment.unix(Number(b.start_time) / 1000).valueOf()
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
          moment.unix(Number(a.start_time) / 1000).valueOf() > moment.unix(Number(b.start_time) / 1000).valueOf()
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
                key={item.title + item.description + item.start_time + item.end_time}
                initial={{ opacity: 0 }}
                whileInView={{ opacity: 1 }}
                viewport={{ once: false }}>
                <Link to={`/stream/${item.id}`} key={item.title + item.description + item.start_time + item.end_time}>
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
