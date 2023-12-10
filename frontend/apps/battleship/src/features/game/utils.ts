import { ADDRESS } from '@/app/consts';
import { initVoucher } from '@dapps-frontend/gasless-transactions';

type ShipLayout = ('Empty' | 'Ship')[];

export const getShipLayout = (shipStatusArray: string[]): number[][] => {
  const shipLayout: number[][] = [];

  let currentShip: number[] = [];
  shipStatusArray.forEach((status, index) => {
    if (status === 'Ship' || status === 'BoomShip') {
      currentShip.push(index);
    } else if (currentShip.length > 0) {
      shipLayout.push(currentShip);
      currentShip = [];
    }
  });

  if (currentShip.length > 0) {
    shipLayout.push(currentShip);
  }

  return shipLayout;
};

export function convertShipsToField(shipPositions: number[][], rows: number, cols: number): ShipLayout {
  const field: ShipLayout = Array.from({ length: rows * cols }, () => 'Empty');

  shipPositions.forEach((ship) => {
    ship.forEach((position) => {
      field[position] = 'Ship';
    });
  });

  return field;
}

export const getFormattedTime = (time: number) => {
  const minutes = Math.floor(time / (1000 * 60));
  const seconds = Math.floor((time % (1000 * 60)) / 1000);

  const formattedTime = `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;

  return formattedTime;
};

export const { useFetchVoucher } = initVoucher({
  programId: ADDRESS.GAME,
  backendAddress: ADDRESS.BACK,
});
