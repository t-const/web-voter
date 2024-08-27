import Create from './Create';
import Home from './Home';
import UserDetails from './UserDetails';
import Navbar from './Navbar';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import RoomDetails from './RoomDetails';

function App() {
  return (
    <Router>
      <div className="App">
        <Navbar />
        <div className="content">
          <Routes>
            <Route exact path="/" element={<Home />}></Route>
            <Route path="/create" element={<Create />}></Route>
            <Route path="/users/:id" element={<UserDetails />}></Route>
            <Route path="/rooms/:id" element={<RoomDetails />}></Route>
          </Routes>
        </div>
      </div>
    </Router>
  );
}

export default App;
