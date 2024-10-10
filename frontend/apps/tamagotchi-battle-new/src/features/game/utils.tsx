import React, { useEffect, useState } from 'react';
import { AssetType } from './types';
import { assetsCount, back_colors, body_colors, CHARACTER_ASSETS_PATH } from './consts';
import { CharacterView } from './components/character/character';

export const getLazySvg = (assetType: AssetType, index: number) => {
  const hasAsset = index >= 0 && index < assetsCount[assetType];
  const assetNumber = hasAsset ? index + 1 : 1;

  return React.lazy(() =>
    import(`${CHARACTER_ASSETS_PATH}${assetType}-${assetNumber}.svg`).then((module) => ({
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