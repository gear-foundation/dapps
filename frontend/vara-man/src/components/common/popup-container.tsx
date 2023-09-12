import { Dialog } from '@headlessui/react'
import { AnimatePresence, motion } from 'framer-motion'
import { Dispatch, ReactNode, SetStateAction } from 'react'
import { cn } from '@/app/utils'
import { XIcon } from 'lucide-react'

export type PopupContainerProps = BaseComponentProps & {
  setIsOpen: Dispatch<SetStateAction<boolean>>
  isOpen: boolean
  overlayCn?: string
  footer?: ReactNode
  title?: string
  panelWidth?: string
}

export function PopupContainer({
  children,
  isOpen,
  setIsOpen,
  overlayCn,
  footer,
  panelWidth,
  title,
  className,
}: PopupContainerProps) {
  return (
    <AnimatePresence>
      {isOpen && (
        <Dialog
          open={isOpen}
          onClose={setIsOpen}
          as="div"
          className="fixed inset-0 z-10 flex items-center justify-center"
        >
          <Dialog.Overlay
            className={cn(
              'fixed inset-0 transition-colors',
              overlayCn ? overlayCn : 'bg-black/5 backdrop-blur'
            )}
          />

          <div className="flex flex-col w-full">
            <motion.div
              className="flex items-center justify-center min-h-screen p-4"
              initial={{
                opacity: 0,
                scale: 0.75,
              }}
              animate={{
                opacity: 1,
                scale: 1,
                transition: {
                  ease: 'easeOut',
                  duration: 0.15,
                },
              }}
              exit={{
                opacity: 0,
                scale: 0.75,
                transition: {
                  ease: 'easeIn',
                  duration: 0.15,
                },
              }}
            >
              <span
                className="hidden sm:inline-block sm:align-middle sm:h-screen"
                aria-hidden="true"
              >
                &#8203;
              </span>

              <Dialog.Panel
                className={cn(
                  'grid w-full bg-[#29292b] shadow-xl rounded-[20px] border-2 border-primary/50 transform-gpu',
                  panelWidth ?? 'max-w-md'
                )}
              >
                <div className="flex items-center justify-between py-4.5 px-8 bg-[#222225] rounded-t-[18px]">
                  <Dialog.Title
                    as="h3"
                    className="text-white/80 text-2xl font-bold leading-9"
                  >
                    {title}
                  </Dialog.Title>
                  <div className="flex -mr-1.5 ml-auto">
                    <button
                      className="text-white/80 hover:text-white transition-colors"
                      onClick={() => setIsOpen((_) => !_)}
                      type="button"
                    >
                      <XIcon className="w-6 h-6" />
                    </button>
                  </div>
                </div>

                <div className="grow p-8">{children}</div>
                {footer}
              </Dialog.Panel>
            </motion.div>
          </div>
        </Dialog>
      )}
    </AnimatePresence>
  )
}
