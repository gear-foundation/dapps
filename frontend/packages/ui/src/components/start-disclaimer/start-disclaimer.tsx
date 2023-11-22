import cx from 'clsx';

import { RELEASE_PATH, WIKI_PATH } from './consts';
import styles from './start-disclaimer.module.css';

type Props = {
  fileName: string;
  wikiPath: string;
  className?: string;
};

function StartDisclaimer({ fileName, wikiPath, className }: Props) {
  const wasmHref = `${RELEASE_PATH}/${fileName}.opt.wasm`;
  const metaHref = `${RELEASE_PATH}/${fileName}.meta.txt`;
  const wikiHref = `${WIKI_PATH}/${wikiPath}/#how-to-run`;

  return (
    <div className={cx(styles.disclaimer, className)}>
      <h3 className={styles.heading}>Get Started</h3>

      <p className={styles.text}>
        To quickly get started, download the default Wasm program of the game, upload it to the blockchain network, and
        then copy its address to specify it in the game
      </p>

      <div className={styles.links}>
        <div>
          <a href={wasmHref} target="_blank" rel="noreferrer">
            Download Program
          </a>

          <a href={metaHref} target="_blank" rel="noreferrer" className={styles.metadataLink}>
            Metadata (optional)
          </a>
        </div>

        <a href={wikiHref} target="_blank" rel="noreferrer">
          How Does It Work?
        </a>
      </div>
    </div>
  );
}

export { StartDisclaimer };
