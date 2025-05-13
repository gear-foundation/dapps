import styles from './menu-button.module.scss';

type Props = {
  title: string;
  subtitle: string;
  onClick: () => void;
  illustration: string;
};

const MenuButton = ({ title, subtitle, onClick, illustration }: Props) => {
  return (
    <button className={styles.menuButton} onClick={onClick}>
      <span className={styles.title}>{title}</span>
      <span className={styles.subtitle}>{subtitle}</span>
      <img src={illustration} alt="illustration" className={styles.illustration} />
    </button>
  );
};

export { MenuButton };
