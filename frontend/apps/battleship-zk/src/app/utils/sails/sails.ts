import { Sails, getServiceNamePrefix } from 'sails-js';
import { GearApi } from '@gear-js/api';
import { ADDRESS } from '@/app/consts';
import idl from '../../../features/game/assets/idl/battleship.idl?raw';

const initSails = async () => {
  const api = await GearApi.create({ providerAddress: ADDRESS.NODE });
  const sails = await Sails.new();

  sails.parseIdl(idl.trim());
  sails.setApi(api);
  sails.setProgramId(ADDRESS.GAME);

  return sails;
};

const sails = await initSails();

export { sails, getServiceNamePrefix };
