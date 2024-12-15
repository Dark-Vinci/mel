import React from 'react';
import './App.css';
import { useDocumentTitle } from "@uidotdev/usehooks";

function App() {
  useDocumentTitle('melon');
  return (
    <div className="App">
      the content
    </div>
  );
}

export default App;
