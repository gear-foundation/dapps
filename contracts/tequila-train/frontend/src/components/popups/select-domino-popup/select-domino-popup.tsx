import { AnimatePresence, motion } from 'framer-motion';
import { Dialog } from '@headlessui/react';
import * as React from 'react';
import { Dispatch, SetStateAction } from 'react';

type Props = {
  setIsOpen: Dispatch<SetStateAction<boolean>>;
  isOpen: boolean;
};

export const SelectDominoPopup = ({ setIsOpen, isOpen }: Props) => {
  return (
    <AnimatePresence>
      {isOpen && (
        <Dialog
          open={isOpen}
          onClose={setIsOpen}
          as="div"
          className="fixed inset-0 z-10 flex items-center justify-center">
          <Dialog.Overlay className="fixed inset-0 bg-black bg-opacity-60 backdrop-blur transition-colors" />

          <div className="flex flex-col">
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
              }}>
              <span className="hidden sm:inline-block sm:align-middle sm:h-screen" aria-hidden="true">
                &#8203;
              </span>

              <div
                className="w-full max-w-md transform overflow-hidden rounded-2xl bg-white p-6 border-4 border-primary text-left align-middle shadow-xl transition-all"
                role="dialog"
                aria-modal="true"
                aria-labelledby="modal-headline">
                <div className="">
                  <Dialog.Title as="h3" className="text-lg leading-6 font-semibold text-dark-500" id="modal-headline">
                    Select domino
                  </Dialog.Title>
                  <div className="mt-2">
                    <Dialog.Description as="p" className="text-sm text-dark-400">
                      To make a turn you should select a domino from the row below. After that choose a proper train
                      slot, and then confirm your move.
                    </Dialog.Description>
                    <div className="mt-4">
                      <button type="button" tabIndex={0} className="btn btn--primary" onClick={() => setIsOpen(false)}>
                        Got it, thanks!
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </motion.div>
          </div>
        </Dialog>
      )}
    </AnimatePresence>
  );
};
