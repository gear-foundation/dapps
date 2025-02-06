import { decodeAddress } from '@gear-js/api';
import React from 'react';

import { CharacterView } from './components/character/character';
import { assetsCount, back_colors, body_colors } from './consts';
import { AssetType } from './types';

export const getLazySvg = (assetType: AssetType, index: number) => {
  const assetNumber = index > 0 ? (index % assetsCount[assetType]) + 1 : 1;

  return React.lazy(() =>
    import(`./assets/images/character/${assetType}-${assetNumber}.svg`).then((module) => ({
      default: module.ReactComponent,
    })),
  );
};

export const getRandomNumber = (maxNumber: number) => Math.floor(Math.random() * maxNumber);

export const generateRandomCharacterView = (): CharacterView => ({
  hat_index: getRandomNumber(assetsCount.hat),
  head_index: getRandomNumber(assetsCount.head),
  body_index: getRandomNumber(assetsCount.body),
  accessory_index: getRandomNumber(assetsCount.accessories),
  body_color: body_colors[getRandomNumber(body_colors.length)],
  back_color: back_colors[getRandomNumber(back_colors.length)],
});

export const getSafeDecodedAddress = (address?: string) => {
  if (address) {
    try {
      return decodeAddress(address.trim());
    } catch (error) {
      // empty
    }
  }
  return null;
};
