import { TamagotchiColor } from 'app/types/battles';

export const getTamagotchiColor = (color: TamagotchiColor): { body: string; sneakers: string } => {
  switch (color) {
    case 'Green':
      return { body: 'text-[#16B768]', sneakers: 'text-[#50468F]' };
    case 'Orange':
      return { body: 'text-[#CF6436]', sneakers: 'text-[#F2D190]' };
    case 'Yellow':
      return { body: 'text-[#DECA13]', sneakers: 'text-[#505351]' };
    case 'Purple':
      return { body: 'text-[#8316B7]', sneakers: 'text-[#B71663]' };
    case 'Red':
      return { body: 'text-[#E34675]', sneakers: 'text-[#1852FF]' };
    case 'Blue':
      return { body: 'text-[#16ADB7]', sneakers: 'text-[#2253FF]' };
    default:
      return { body: 'text-[#16B768]', sneakers: 'text-[#50468F]' };
  }
};
