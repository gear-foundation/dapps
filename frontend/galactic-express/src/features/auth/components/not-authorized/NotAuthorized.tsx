import { Link } from 'react-router-dom';
import { cx } from 'utils';
import styles from './NotAuthorized.module.scss';

function NotAuthorized() {
  const testnetLink = process.env.REACT_APP_TESTNET_URL || '';

  return (
    <div className={cx(styles.container)}>
      <div className={cx(styles.content)}>
        <div className={cx(styles.header)}>
          <h1 className={cx(styles['heading-title'], styles[`heading-title-with-gradient`])}>Rockets game</h1>
          <div>
            You are currently not part of the Vara Network Testnet. Please register using the referral link in the
            Testnet portal:
          </div>
        </div>
        <a href={testnetLink} target="_blank" rel="noreferrer" className={cx(styles['testnet-link'])}>
          Vara Network Testnet
        </a>
        <div className={cx(styles.bottom)}>
          More information can be found in our
          <Link to="https://discord.gg/x8ZeSy6S6K" target="_blank" className={styles.link}>
            Discord
          </Link>
          and
          <Link to="https://t.me/VaraNetwork_Global" target="_blank" className={styles.link}>
            Telegram
          </Link>
        </div>
      </div>
    </div>
  );
}

export { NotAuthorized };
