import { AnimatePresence, motion } from 'framer-motion';
import { Dialog } from '@headlessui/react';
import * as React from 'react';
import { Dispatch, SetStateAction } from 'react';
import { Icon } from '../../ui/icon';

type Props = {
  setIsOpen: Dispatch<SetStateAction<boolean>>;
  isOpen: boolean;
};

export const WinnerPopup = ({ setIsOpen, isOpen }: Props) => {
  return (
    <AnimatePresence>
      {isOpen && (
        <Dialog
          open={isOpen}
          onClose={setIsOpen}
          as="div"
          className="fixed inset-0 z-10 flex items-center justify-center">
          <Dialog.Overlay className="fixed inset-0 bg-black bg-opacity-90 backdrop-blur transition-colors" />

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

              <div className="flex items-center pt-40">
                <div
                  className="relative w-full max-w-[720px] transform transition-all"
                  role="dialog"
                  aria-modal="true"
                  aria-labelledby="modal-headline">
                  <img
                    src="/images/winner.svg"
                    alt="Winner"
                    className="absolute bottom-[80%] left-1/2 -translate-x-1/2 h-[175%]"
                  />
                  <img
                    src="/images/winner-bg.svg"
                    alt="Winner"
                    className="absolute bottom-[60%] -z-1 left-1/2 -translate-x-1/2 h-[200%]"
                  />
                  <div className="rounded-2xl bg-white px-8 py-12 xxl:pt-12 xxl:pb-19 border-6 border-[#1E942A] shadow-xl">
                    <Dialog.Title
                      as="h3"
                      className="text-[40px] xxl:text-[48px] leading-18 font-bold text-center text-transparent-primary"
                      id="modal-headline">
                      Congrats!
                    </Dialog.Title>
                    <div className="mt-2">
                      <Dialog.Description
                        as="p"
                        className="text-lg xxl:text-[21px] leading-5 mt-6 text-center text-dark-500 font-extrabold tracking-[0.08em]">
                        <span className="text-[#00D1FF]">Señor Azul</span> is a winner! Take your tequila and enjoy!
                      </Dialog.Description>
                      <div className="absolute bottom-0 left-1/2 mt-4 w-[200px] w-[255px] -translate-x-1/2 translate-y-1/2">
                        <button
                          type="button"
                          tabIndex={0}
                          className="btn btn--primary w-full text-base xxl:text-xl text-dark-500 font-semibold"
                          onClick={() => setIsOpen(false)}>
                          OK
                        </button>
                      </div>
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
