import { MouseEvent, ReactNode, useEffect, useRef } from 'react'
import { Button } from '@gear-js/ui'
import { motion, Variants } from 'framer-motion'
import styles from './Modal.module.scss'
import { CrossIcon } from '@/assets/images'

export const variantsOverlay: Variants = {
  enter: { opacity: 0 },
  center: {
    opacity: 1,
    transition: {
      ease: 'easeOut',
      duration: 0.3,
    },
  },
  exit: {
    opacity: 0,
    transition: {
      ease: 'easeIn',
      duration: 0.2,
    },
  },
}
export const variantsPanel: Variants = {
  enter: {
    opacity: 0,
    scale: 0.75,
  },
  center: {
    opacity: 1,
    scale: 1,
    transition: {
      ease: 'easeOut',
      duration: 0.3,
    },
  },
  exit: {
    opacity: 0,
    scale: 0.75,
    transition: {
      ease: 'easeIn',
      duration: 0.2,
    },
  },
}

type Props = {
  heading: string
  children: ReactNode
  onClose: () => void
}

export function Modal({ heading, children, onClose }: Props) {
  const ref = useRef<HTMLDialogElement>(null)

  const disableScroll = () => document.body.classList.add('modal-open')
  const enableScroll = () => document.body.classList.remove('modal-open')

  const open = () => {
    ref.current?.showModal()
    disableScroll()
  }

  const close = () => {
    ref.current?.close()
    enableScroll()
  }

  useEffect(() => {
    open()

    return () => close()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  const handleClick = ({ target }: MouseEvent) => {
    const isBackdropClick = target === ref.current

    if (isBackdropClick) onClose()
  }

  return (
    <motion.dialog
      initial="enter"
      animate="center"
      exit="exit"
      variants={variantsOverlay}
      ref={ref}
      onClick={handleClick}
      className={styles.modal}
    >
      <motion.div
        initial="enter"
        animate="center"
        exit="exit"
        variants={variantsPanel}
        className={styles.wrapper}
      >
        <div className={styles.header}>
          <h2 className={styles.header__title}>{heading}</h2>

          <Button icon={CrossIcon} color="transparent" onClick={onClose} />
        </div>

        {children}
      </motion.div>
    </motion.dialog>
  )
}
