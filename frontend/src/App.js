import logo from './logo.svg';
import './App.css';
import React, { useState } from 'react';

const user = {
  name: 'Hedy Lamarr',
  imageUrl: 'https://i.imgur.com/yXOvdOSs.jpg',
  imageSize: 90,
};

export default function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        <h1>This is MY body.</h1>
        <Profile />
        <WtfIsThatList />
      </header>
    </div>
  );
}

function Profile() {
  const [isLoggedIn, setAdmin] = useState(false);

  const handleButtonClick = () => {
    setAdmin(!isLoggedIn);
  }

  return (
    <div>
      {isLoggedIn ? (
        <AdminProfile />
      ) : (
        <LoginProfile />
      )}
      <h1>
        <p>{isLoggedIn}</p>
        <MyButton className="MyButton" onClick={handleButtonClick} />
      </h1>
    </div>
  );
}

function LoginProfile() {
  return (
    <>
      <h1>{user.name}</h1>
      <img
        className="avatar"
        src={user.imageUrl}
        alt={'Photo of ' + user.name}
        style={{
          width: user.imageSize,
          height: user.imageSize
        }}
      />
    </>
  );
}

function AdminProfile() {
  return (
    <>
      <h1>{user.name} - God</h1>
      <img
        className="avatar"
        src='https://i.imgur.com/6amXGTM.jpeg'
        alt={'Photo of ' + user.name}
        style={{
          width: user.imageSize,
          height: user.imageSize
        }}
      />
    </>
  );
}

function MyButton(props) {
  return (
    <>
      <button className="App-button" id='buttonId' onClick={props.onClick}>I'm a button, who would've thought.</button>
    </>
  )
}

function WtfIsThatList() {
  const someVeryUsefulList = [
    { title: 'Method 1', id: 1 },
    { title: 'Method 2', id: 2 },
    { title: 'Method 3', id: 3 }
  ]

  const listItems = someVeryUsefulList.map(item =>
    <li key={item.id}>
      {item.title}
    </li>
  );

  return (
    <ul>{listItems}</ul>
  )
}

