import { useRef, useState } from 'react';
import { cx, shortenString } from '@/utils';
import styles from './ContractInfo.module.scss';
import selectArrow from '@/assets/icons/select-arrow.svg';
import { ReactComponent as AvaVara } from '@/assets/icons/ava-vara-black.svg';
import { ReactComponent as ContractAddress } from '@/assets/icons/contract-address-rounded.svg';
import { ReactComponent as PenEdit } from '@/assets/icons/ic-edit-24.svg';
import { useClickOutside } from '@/hooks';
import { NodeSwitch } from '@/features/NodeSwitch';
import { Chain } from '@/features/NodeSwitch/components/NodeSwitch/NodeSwitch.interfaces';

function ContractInfo() {
  const [open, setOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);
  const dropdownRef = useRef<HTMLButtonElement>(null);
  const [currentChain, setCurrentChain] = useState<Chain>(null);

  const handleChangeChain = (chain: Chain) => {
    setCurrentChain(chain);
  };

  useClickOutside(
    () => {
      setOpen(false);
    },
    menuRef,
    dropdownRef,
  );

  return (
    <div className={cx(styles.container)}>
      <button onClick={() => setOpen(!open)} className={cx(styles.dropdown)} ref={dropdownRef}>
        <AvaVara className={cx(styles['dropdown-label'])} />
        <img
          src={selectArrow}
          alt="chevron"
          className={cx(styles['dropdown-toggle-arrow'], open ? styles['dropdown-toggle-arrow-rotated'] : '')}
        />
      </button>

      {open && (
        <div className={cx(styles['dropdown-menu'])} ref={menuRef}>
          <div className={cx(styles.item)}>
            <ContractAddress className={cx(styles['item-prefix-icon'])} />
            <div className={cx(styles['item-content'])}>
              <span className={cx(styles['item-title'])}>Contract address</span>
              <span className={cx(styles['item-value'])}>{shortenString('0x256cd4...155068cd', 8)}</span>
            </div>
            <PenEdit className={cx(styles['item-edit-icon'])} />
          </div>
          <NodeSwitch onChainChange={handleChangeChain}>
            {currentChain ? (
              <div className={cx(styles.item)}>
                <currentChain.icon className={cx(styles['item-prefix-icon'])} />
                <div className={cx(styles['item-content'])}>
                  <span className={cx(styles['item-title'])}>{currentChain.name}</span>
                  <span className={cx(styles['item-value'])}>{currentChain.address}</span>
                </div>
                <PenEdit className={cx(styles['item-edit-icon'])} />
              </div>
            ) : (
              <></>
            )}
          </NodeSwitch>
        </div>
      )}
    </div>
  );
}

export { ContractInfo };
