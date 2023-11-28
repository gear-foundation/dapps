import './App.css';
import { Route, Routes } from 'react-router-dom';
import Main from './components/Main';
import Watch from './components/Watch';
import Braodcast from './components/Broadcast';

function App() {
  return (
    <div>
      <Routes>
        <Route path="/" element={<Main />}></Route>
        <Route path="watch" element={<Watch />}></Route>
        <Route path="broadcast" element={<Braodcast />}></Route>
      </Routes>
    </div>
  );
}

export default App;
