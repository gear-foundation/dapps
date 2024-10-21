import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';
import { Program as VaratubeProgram } from './varatube';
import { Program as VftProgram } from './extended_vft';
import { ADDRESS } from 'consts';

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
