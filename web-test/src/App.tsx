import React, { useState, useEffect} from 'react';
import logo from './logo.svg';
import './App.css';

function App() {

  let url = "http://localhost:8000/";
  const api_get = () => fetch(url).then(
    (e)=> console.log(e)
  );

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={api_get}>Fetch API</button>
      </header>
    </div>
  );
}

export default App;
