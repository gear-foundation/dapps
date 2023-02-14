import { PlayerRowSection } from '../player-row-section';
import { PlayerCardSection } from '../player-card-section';
import { PlayerConsSection } from '../player-cons-section';

const players = ['Rojo', 'Oscuro', 'Naranja', 'Amarillo', 'Gris', 'Verde', 'Azul', 'Morado'];
export const GameSection = () => {
  return (
    <div className="container-xl flex flex-col grow">
      <ul className="space-y-px">
        <li>
          <PlayerRowSection index={-1} train />
        </li>
        {players.map((p, i) => (
          <li key={i}>
            <PlayerRowSection index={i} />
          </li>
        ))}
      </ul>
      <div className="grid gap-4 mt-auto">
        <PlayerConsSection />
        <ul className="flex gap-4">
          {players.map((p, i) => (
            <li key={i}>
              <PlayerCardSection index={i} />
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};
