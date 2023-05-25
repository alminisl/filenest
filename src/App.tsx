import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

interface Folders {
  name: string;
  path: string;
  extensions: string[];
}

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [basePath, setBasePath] = useState("");
  const [folders, setFolders] = useState<Folders[]>([]);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  async function updateConfig() {
    console.log("updateConfig: ", { basePath, folders });
    await invoke("updateConfig", { basePath, folders });
  }

  useEffect(() => {
    setBasePathValue();
    setFoldersValue();
  }, []);

  async function setBasePathValue() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const basePath = await invoke("getBasePath");
    setBasePath(basePath as any);
  }

  async function setFoldersValue() {
    const folders = await invoke("getFolders");
    console.log("folders: ", folders);
    setFolders(folders as Folders[]);
  }

  return (
    <div className="container">
      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            // greet();
            updateConfig();
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
            <div key={folder.name}>
              <label htmlFor="folder-input">{folder.name}</label>
              <input
                id="folder-input"
                onChange={(e) => {
                  const newFolders = folders.map((f) => {
                    if (f.name === folder.name) {
                      return {
                        ...f,
                        path: e.currentTarget.value,
                      };
                    }
                    return f;
                  });
                  setFolders(newFolders);
                }}
                value={folder.path}
              />
              <input
                id="folder-extensoins"
                // onChange={(e) => }
                value={folder.extensions.join(",")}
              ></input>
            </div>
          ))}
        </form>
      </div>
      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
