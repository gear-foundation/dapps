import { cx } from 'utils';
import { Button } from 'components/layout/button';
import styles from './GetStarted.module.scss';

function GetStarted() {
  return (
    <div className={cx(styles.container)}>
      <div className={cx(styles.title)}>get started</div>
      <div className={cx(styles.text)}>
        To quickly get started, download the default Wasm program of the game, upload it to the blockchain network, and
        then copy its address to specify it in the game
      </div>
      <div className={cx(styles.controls)}>
        <div>
          <a
            href="https://github.com/gear-foundation/dapps/releases/download/nightly/galactic_express.opt.wasm"
            target="_blank"
            rel="noreferrer">
            <Button label="Download program" variant="text" className={cx(styles.control)} />
          </a>
          <a
            href="https://github.com/gear-foundation/dapps/releases/download/nightly/galactic_express.meta.txt"
            target="_blank"
            rel="noreferrer">
            <Button label="Metadata ( optional )" variant="text" className={cx(styles.control, styles.metadata)} />
          </a>
        </div>
        <a
          href="https://wiki.gear-tech.io/docs/examples/Gaming/galactic-express#how-to-run"
          target="_blank"
          rel="noreferrer">
          <Button label="How does it work?" variant="text" className={cx(styles.control)} />
        </a>
      </div>
    </div>
  );
}

export { GetStarted };
