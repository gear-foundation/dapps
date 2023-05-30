import { atom } from 'jotai';
import { ADDRESS, LOCAL_STORAGE } from 'consts';
import { ReactComponent as GearNetworkSVG } from './assets/gear.svg';
import { ReactComponent as NetworkVaraSVG } from './assets/vara.svg';

const DEVELOPMENT_SECTION = 'development';

const ICON = {
  vara: NetworkVaraSVG,
  gear: GearNetworkSVG,
};

const NODE_ADDRESS_ATOM = atom((localStorage[LOCAL_STORAGE.NODE] as string) || ADDRESS.DETAULT_NODE);

export { DEVELOPMENT_SECTION, ICON, NODE_ADDRESS_ATOM };
