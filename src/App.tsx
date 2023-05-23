import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [basePath, setBasePath] = useState("");
  const [folders, setFolders] = useState([]);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  useEffect(() => {
    setBasePathValue();
    setFoldersValue();
  }, []);

  async function setBasePathValue() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const basePath = await invoke("getBasePath");
    console.log("basePath: ", basePath);
    setBasePath(basePath as any);
  }

  async function setFoldersValue() {
    const folders = await invoke("getFolders");
    console.log("folders: ", folders);
    setFolders(folders as any);
  }

  return (
    <div className="container">
      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="submit">Greet</button>

          <input
            id="base-path-input"
            onChange={(e) => setBasePath(e.currentTarget.value)}
            value={basePath}
          />

          {folders.map((folder) => (
            <div key={folder}>
              <input
                id="folder-input"
                onChange={(e) => setBasePath(e.currentTarget.value)}
                value={folder}
              />
            </div>
          ))}
        </form>
      </div>
      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
