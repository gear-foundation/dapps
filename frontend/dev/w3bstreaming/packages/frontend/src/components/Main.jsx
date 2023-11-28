import { Link } from 'react-router-dom';

export default function Main() {
  return (
    <div className="Main">
      <nav>
        <Link to="watch">Watch</Link>
        <br />
        <Link to="broadcast">Broadcast</Link>
      </nav>
    </div>
  );
}
