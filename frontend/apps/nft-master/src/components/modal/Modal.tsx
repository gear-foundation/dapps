import { ReactNode, useEffect, useRef, MouseEvent } from 'react'
import { ReactComponent as CrossSVG } from 'assets/images/icons/cross.svg'
import { useResizeEffect } from 'hooks'
import styles from './Modal.module.scss'
import { Button } from '../ui/button'
import { Sprite } from '../ui/sprite'

type Props = {
  heading: string
  children: ReactNode
  onClose: () => void
}

function Modal({ heading, children, onClose }: Props) {
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

  // close on resize, cuz mobile layout has duplicate modal execution components
  useResizeEffect(onClose)

  const handleClick = ({ target }: MouseEvent) => {
    const isBackdropClick = target === ref.current

    if (isBackdropClick) onClose()
  }

  return (
    // eslint-disable-next-line jsx-a11y/click-events-have-key-events, jsx-a11y/no-noninteractive-element-interactions
    <dialog ref={ref} onClick={handleClick} className={styles.modal}>
      <div className={styles.wrapper}>
        <header className={styles.header}>
          <h2>{heading}</h2>

          <Button
            variant="text"
            onClick={onClose}
            className={styles['modal-close']}
          >
            <Sprite name="close" width={25} height={24} />
          </Button>
        </header>

        {children}
      </div>
    </dialog>
  )
}

export { Modal }
