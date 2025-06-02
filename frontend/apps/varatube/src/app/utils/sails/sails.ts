import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';

import { ENV } from '@/consts';

import { Program as VftProgram } from './extended_vft';
import { Program as VaratubeProgram } from './varatube';

const useVaratubeProgram = () => {
  const { data: program } = useGearJsProgram({
    library: VaratubeProgram,
    id: ENV.CONTRACT,
  });

  return program;
};

const useVftProgram = () => {
  const { data: program } = useGearJsProgram({
    library: VftProgram,
    id: ENV.FT_CONTRACT,
  });

  return program;
};

export { useVaratubeProgram, useVftProgram };
