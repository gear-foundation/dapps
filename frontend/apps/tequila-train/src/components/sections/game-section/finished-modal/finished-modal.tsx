
import { useApp } from 'app/context';

import { WinnerPopup } from 'components/popups/winner-popup/winner-popup';

export function FinishedSection() {
  const { openWinnerPopup, setOpenWinnerPopup } = useApp();

  return (
    <WinnerPopup isOpen={openWinnerPopup} setIsOpen={setOpenWinnerPopup} />
  );
}
