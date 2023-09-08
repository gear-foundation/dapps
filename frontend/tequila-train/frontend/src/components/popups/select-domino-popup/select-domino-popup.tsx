import { Dialog } from '@headlessui/react';
import { Dispatch, SetStateAction } from 'react';
import { PopupContainer } from '../popup-container';

type Props = {
  setIsOpen: Dispatch<SetStateAction<boolean>>;
  isOpen: boolean;
};

export const SelectDominoPopup = ({ setIsOpen, isOpen }: Props) => {
  return (
    <PopupContainer isOpen={isOpen} setIsOpen={setIsOpen}>
      <div className="w-full max-w-md transform overflow-hidden rounded-2xl bg-white p-6 border-4 border-primary text-left align-middle shadow-xl transition-all">
        <div>
          <Dialog.Title as="h3" className="text-lg leading-6 font-semibold text-dark-500" id="modal-headline">
            Select domino
          </Dialog.Title>
          <div className="mt-2">
            <Dialog.Description as="p" className="text-sm text-dark-400">
              To make a turn you should select a domino from the row below. After that choose a proper train slot, and
              then confirm your move.
            </Dialog.Description>
            <div className="mt-4">
              <button type="button" tabIndex={0} className="btn btn--primary" onClick={() => setIsOpen(false)}>
                Got it, thanks!
              </button>
            </div>
          </div>
        </div>
      </div>
    </PopupContainer>
  );
};
