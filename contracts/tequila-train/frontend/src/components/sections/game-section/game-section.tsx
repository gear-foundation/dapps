import { PlayerRowSection } from '../player-row-section';

const players = ['Rojo', 'Oscuro', 'Naranja', 'Amarillo', 'Gris', 'Verde', 'Azul', 'Morado'];
export const GameSection = () => {
  return (
    <div className="container-xl">
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
    </div>
  );
};
