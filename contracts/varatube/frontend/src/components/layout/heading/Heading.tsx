import styles from './Heading.module.scss';

type Props = {
  text: string;
};

function Heading({ text }: Props) {
  return <h2 className={styles.heading}>{text}</h2>;
}

export { Heading };
