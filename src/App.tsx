import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import logo from "../icon/icon.png";

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
    const object = JSON.stringify({ basePath, folders });
    await invoke("updateConfig", { invoke_message: object });
  }

  async function sortItems() {
    await invoke("sortItems");
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
    setFolders(folders as Folders[]);
  }

  return (
    <>
      <div className="logo">
        <img src={logo} alt="Logo" className="logo-image" />
        <label className="logo-label">Filenest</label>
      </div>

      <div className="container">
        <div className="row">
          <form
            onSubmit={(e) => {
              e.preventDefault();
            }}
          >
            <div className="input-container">
              <label htmlFor="base-path-input">Base path</label>
              <input
                id="base-path-input"
                onChange={(e) => setBasePath(e.currentTarget.value)}
                value={basePath}
                disabled={true}
              />
            </div>

            {folders.map((folder) => (
              <div key={folder.name} className="input-container">
                <label htmlFor="folder-input" className="folder-label">
                  {folder.name}
                </label>
                <div className="input-background">
                  <input
                    id="folder-input"
                    className="folder-input"
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
                  <label htmlFor="folder-extension" className="folder-label">
                    Extension:
                  </label>
                  <input
                    id="folder-extensions"
                    className="folder-extensions"
                    value={folder.extensions.join(",")}
                    onChange={(e) => {
                      const newFolders = folders.map((f) => {
                        if (f.name === folder.name) {
                          return {
                            ...f,
                            extensions: e.currentTarget.value.split(","),
                          };
                        }
                        return f;
                      });
                      setFolders(newFolders);
                    }}
                  />
                </div>
              </div>
            ))}
            <div>
              <button>PLACEHOLDER: ADD NEW FOLDER</button>
            </div>
            <button onClick={(e) => updateConfig()} className="save-button">
              Save
            </button>
            <button onClick={(e) => sortItems()}>Re-sort</button>
          </form>
        </div>
      </div>
    </>
  );
}

export default App;
