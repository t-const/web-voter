import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  // using strict mode execute some functions twice.
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
