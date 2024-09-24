import styles from './Copyright.module.scss';

function Copyright() {
  const year = new Date().getFullYear();

  return <small className={styles.copyright}>&copy; {year} Gear Foundation, Inc. All rights reserved.</small>;
}

export { Copyright };
