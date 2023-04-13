/**
 * Main file where electron and its components are being loaded. This is also
 * the place where the messages are being handled for now.
 *
 * @summary Main file
 * @author
 *
 * Created at     : 2021-07-20 08:07:03
 * Last modified  : 2023-04-13 18:03:21
 */

const {
  app,
  Menu,
  Tray,
  dialog,
  BrowserWindow,
  ipcMain,
  nativeImage,
} = require("electron");
const path = require("path");
const url = require("url");
const manager = require("./manager");
const log = require("electron-log");

const { config, init, saveToConfig, setDownloadPath } = require("./config");
const { settings } = require("cluster");
app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

if (handleSquirrelEvent(app)) {
  // squirrel event handled and app will exit in 1000ms, so don't do anything else
  return;
}

let tray = null;
let settingsWindow;

try {
  app.whenReady().then(() => {
    const icon = "/icon/icon.png";
    const pathToIcon = app.getAppPath();
    if (process.platform === "win32") {
      tray = new Tray(pathToIcon + icon);
    } else {
      const image = nativeImage.createFromPath(pathToIcon + icon);
      tray = new Tray(image.resize({ width: 16, height: 16 }));
    }
    init();

    const contextMenu = Menu.buildFromTemplate([
      {
        label: "Settings",
        type: "normal",
        click: () => {
          settingsWindow = new BrowserWindow({
            webPreferences: {
              nodeIntegration: true,
              contextIsolation: false,
            },
            width: 600,
            height: 700,
            icon: "./icon/icon.png",
          });
          settingsWindow.setMenuBarVisibility(false);
          settingsWindow.on("close", function (event) {
            if (!app.isQuiting) {
              event.preventDefault();
              settingsWindow.hide();
            }

            return false;
          });

          settingsWindow.on("minimize", function (event) {
            event.preventDefault();
            settingsWindow.hide();
          });
          settingsWindow.loadURL(
            url.format({
              pathname: path.join(__dirname, "settings.html"),
              protocol: "file:",
              slashes: true,
            })
          );

          ipcMain.on("onload-paths", (event, arg) => {
            const paths = {};
            for (const [key, value] of Object.entries(config)) {
              if (key.includes("PATH")) {
                paths[key] = value;
              }
            }
            console.log("*****PathsToRender: ", paths);
            event.reply("reply-paths", paths);
          });

          ipcMain.on("onload-extensions", (event, arg) => {
            const extensions = {};
            for (const [key, value] of Object.entries(config)) {
              if (key.includes("EXTENSIONS")) {
                extensions[key] = value;
              }
            }

            console.log("Extensions: ", extensions);
            event.reply("reply-extensions", extensions);
          });

          ipcMain.on("saveToConfig", async function (event, paths, extensions) {
            console.log("SaveToConfig: ", paths, extensions);
            // savetoConfig(arg);

            // event.reply("reply-paths", paths);
            // event.reply("reply-extensions", extensions);
          });
        },
      },
      {
        label: "Sort download folder",
        type: "normal",
        click: () => manager.findAllMovableFiles(),
      },
      {
        label: "Watcher",
        type: "checkbox",
        click: () => {
          if (contextMenu.items[2].checked) {
            watcher = manager.watcher();
          } else {
            manager.stopWatcher(watcher);
          }
        },
        checked: true,
      },
      {
        label: "Close",
        type: "normal",
        click: () => {
          app.isQuiting = true;
          app.quit();
        },
      },
    ]);
    tray.setToolTip("Filenest");

    let watcher;
    if (contextMenu.items[2].checked) {
      watcher = manager.watcher();
    } else {
      manager.stopWatcher(watcher);
    }

    tray.setContextMenu(contextMenu);
  });
} catch (e) {
  log.error(e);
  const fs = require("fs");
  // Or
  fs.writeFileSync("/tmp/test-sync", e);
}

/**
 * Used for creating a installation file
 */
function handleSquirrelEvent(application) {
  if (process.argv.length === 1) {
    return false;
  }

  const ChildProcess = require("child_process");
  const path = require("path");

  const appFolder = path.resolve(process.execPath, "..");
  const rootAtomFolder = path.resolve(appFolder, "..");
  const updateDotExe = path.resolve(path.join(rootAtomFolder, "Update.exe"));
  const exeName = path.basename(process.execPath);

  const spawn = function (command, args) {
    let spawnedProcess, error;

    try {
      spawnedProcess = ChildProcess.spawn(command, args, {
        detached: true,
      });
    } catch (error) {}

    return spawnedProcess;
  };

  const spawnUpdate = function (args) {
    return spawn(updateDotExe, args);
  };

  const squirrelEvent = process.argv[1];
  switch (squirrelEvent) {
    case "--squirrel-install":
    case "--squirrel-updated":
      // Optionally do things such as:
      // - Add your .exe to the PATH
      // - Write to the registry for things like file associations and
      //   explorer context menus

      // Install desktop and start menu shortcuts
      spawnUpdate(["--createShortcut", exeName]);

      setTimeout(application.quit, 1000);
      return true;

    case "--squirrel-uninstall":
      // Undo anything you did in the --squirrel-install and
      // --squirrel-updated handlers

      // Remove desktop and start menu shortcuts
      spawnUpdate(["--removeShortcut", exeName]);

      setTimeout(application.quit, 1000);
      return true;

    case "--squirrel-obsolete":
      // This is called on the outgoing version of your app before
      // we update to the new version - it's the opposite of
      // --squirrel-updated

      application.quit();
      return true;
  }
}
