import React from 'react';
import './App.css';
import WasmCanvas from './WasmCanvas';

function App() {
  return (
    <div className="App">
      <div style={{ padding: "64px"}}>
        <WasmCanvas />
      </div>
    </div>
  );
}

export default App;
