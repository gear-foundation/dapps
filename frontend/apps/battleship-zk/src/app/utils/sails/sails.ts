import { GearApi } from '@gear-js/api';
import { ADDRESS } from '@/app/consts';
import { Program } from '@/app/utils/sails/lib/lib';

const initSails = async () => {
  const api = await GearApi.create({ providerAddress: ADDRESS.NODE });
  const program = new Program(api, ADDRESS.GAME);

  return program;
};

const program = await initSails();

export { program };
