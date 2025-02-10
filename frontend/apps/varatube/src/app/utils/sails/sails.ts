import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';

import { ADDRESS } from '@/consts';

import { Program as VftProgram } from './extended_vft';
import { Program as VaratubeProgram } from './varatube';

const useVaratubeProgram = () => {
  const { data: program } = useGearJsProgram({
    library: VaratubeProgram,
    id: ADDRESS.CONTRACT,
  });

  return program;
};

const useVftProgram = () => {
  const { data: program } = useGearJsProgram({
    library: VftProgram,
    id: ADDRESS.FT_CONTRACT,
  });

  return program;
};

export { useVaratubeProgram, useVftProgram };
