import React, { useEffect, useState } from 'react';
import { AssetType } from './types';
import { assetsCount, backColors, bodyColors, CHARACTER_ASSETS_PATH } from './consts';
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
  hatIndex: getRandomNumber(assetsCount.hat),
  headIndex: getRandomNumber(assetsCount.head),
  bodyIndex: getRandomNumber(assetsCount.body),
  accessoryIndex: getRandomNumber(assetsCount.accessories),
  bodyColor: bodyColors[getRandomNumber(bodyColors.length)],
  backColor: backColors[getRandomNumber(backColors.length)],
});
