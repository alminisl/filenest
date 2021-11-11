/**
 * Main file where electron and its components are being loaded. This is also
 * the place where the messages are being handled for now.
 *
 * @summary Main file
 * @author
 *
 * Created at     : 2021-07-20 08:07:03
 * Last modified  : 2021-11-11 16:07:20
 */

const { app, Menu, Tray, dialog, BrowserWindow, ipcMain } = require("electron");
const path = require("path");
const url = require("url");
const manager = require("./manager");
const { config, init, saveToConfig } = require("./config");
const { settings } = require("cluster");
init();
app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

let tray = null;
let settingsWindow;
app.whenReady().then(() => {
  tray = new Tray("./icon/icon.png");

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
          backgroundColor: "#282828",
          width: 600,
          height: 700,
          icon: "./icon/icon.png",
        });
        settingsWindow.setMenuBarVisibility(false);
        settingsWindow.setBackgroundColor("#282828");
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
          const paths = {
            basePath: config.BASE_PATH,
            documentsPath: config.DOCUMENTS_PATH,
            applicationPath: config.APPLICATION_PATH,
            imagesPath: config.IMAGES_PATH,
            archivesPath: config.ARCHIVES_PATH,
          };
          event.reply("reply-paths", paths);
        });

        ipcMain.on("onload-extensions", (event, arg) => {
          const extensions = {
            documentExtensions: config.DOCUMENT_EXTENSIONS,
            applicationextensions: config.APPLICATION_EXTENSIONS,
            imagesExtensions: config.IMAGES_EXTENSIONS,
            archiveExtensions: config.ARCHIVE_EXTENSIONS,
          };
          console.log("Extensions: ", extensions);
          event.reply("reply-extensions", extensions);
        });

        ipcMain.on("selectDirectory", async function (event, arg) {
          dir = dialog.showOpenDialog(settingsWindow, {
            properties: ["openDirectory"],
          });

          const selectedDir = await dir;
          event.reply("newMainFolder", selectedDir.filePaths[0]);
        });

        ipcMain.on("saveToConfig", async function (event, arg) {
          config.BASE_PATH = arg.basePathSetting;
          config.DOCUMENTS_PATH = arg.DocumentPath;
          config.APPLICATION_PATH = arg.ApplicationPath;
          config.IMAGES_PATH = arg.ImagesPath;
          config.ARCHIVES_PATH = arg.ArchivePath;

          config.DOCUMENT_EXTENSIONS = arg.documentX;
          config.APPLICATION_EXTENSIONS = arg.applicationX;
          config.IMAGES_EXTENSIONS = arg.imageX;
          config.ARCHIVE_EXTENSIONS = arg.archiveX;

          saveToConfig(config);

          const paths = {
            basePath: config.BASE_PATH,
            documentsPath: config.DOCUMENTS_PATH,
            applicationPath: config.APPLICATION_PATH,
            imagesPath: config.IMAGES_PATH,
            archivesPath: config.ARCHIVES_PATH,
          };

          const extensions = {
            documentExtensions: config.DOCUMENT_EXTENSIONS,
            applicationextensions: config.APPLICATION_EXTENSIONS,
            imagesExtensions: config.IMAGES_EXTENSIONS,
            archiveExtensions: config.ARCHIVE_EXTENSIONS,
          };

          event.reply("reply-paths", paths);
          event.reply("reply-extensions", extensions);
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
