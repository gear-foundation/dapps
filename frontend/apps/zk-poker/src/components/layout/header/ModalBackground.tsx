import { motion, AnimatePresence } from 'framer-motion';

import styles from './header.module.scss';

type Props = {
  isOpen: boolean;
  onClick: () => void;
};

const ModalBackground = ({ isOpen, onClick }: Props) => {
  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div
          className={styles.modalBackground}
          initial={{ opacity: 0 }}
          animate={{ opacity: 0.6 }}
          exit={{ opacity: 0 }}
          onClick={onClick}
        />
      )}
    </AnimatePresence>
  );
};

export default ModalBackground;
