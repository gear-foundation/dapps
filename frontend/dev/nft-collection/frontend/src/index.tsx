import { createRoot } from 'react-dom/client';
import { App } from '@/App';

import './styles/global.scss';

const container = document.getElementById('root');
const root = createRoot(container as Element);

root.render(<App />);
