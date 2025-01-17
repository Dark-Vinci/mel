import './App.css';
import { useDocumentTitle } from "@uidotdev/usehooks";

export function App() {
  useDocumentTitle('melon');
  return (
    <div className="App">
      the content
    </div>
  );
}

export default App;
