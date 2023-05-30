import { ReactComponent as GearNetworkSVG } from './assets/gear.svg';
import { ReactComponent as NetworkVaraSVG } from './assets/vara.svg';

const DEVELOPMENT_SECTION = 'development';

const GENESIS = {
  VARA_TESTNET: '0x69599490fc00e8c5636ec255f4eee61d1ca950dd87df7a32cd92b6c8f61dbe28',
  VARA: '0xfe1b4c55fd4d668101126434206571a7838a8b6b93a6d1b95d607e78e6c53763',
};

// TODO: think about naming of ICON and LOGO, and overall structure
const ICON = {
  vara: { NETWORK: NetworkVaraSVG },
  gear: { NETWORK: GearNetworkSVG },
};

const LOGO = {
  [GENESIS.VARA_TESTNET]: ICON.vara,
  [GENESIS.VARA]: ICON.vara,
};

export { DEVELOPMENT_SECTION, ICON, LOGO };
