import { CarsState } from '../Road/Road.interface';

export interface CanvasRoadProps {
  cars: CarsState | null;
  carIds: string[];
  imagesCollection: Record<string, HTMLImageElement>;
}
