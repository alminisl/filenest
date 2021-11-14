/**
 * Handles all things related to configuration of the app. Saving, loading and
 * settings the configuration variables is the job of this file and the
 * functions within it.
 *
 * @summary Configuration manager
 * @author
 *
 * Created at     : 2021-07-20 08:05:58
 * Last modified  : 2021-11-14 22:14:34
 */

const fs = require("fs");
const { app } = require("electron");
let config = (async () => loadConfigData())();
console.log("Config loaded: ", config);

let isWindows = false;
let downloadFolder = setDownloadPath(app.getPath("home"));
if (config.BASE_PATH === undefined && config.BASE_PATH !== "") {
  BASE_PATH = downloadFolder;
} else {
  BASE_PATH = config.BASE_PATH;
}

// Check if the system is windows or not, return correct path prefix
function filePathFormat() {
  return isWindows ? "\\" : "/";
}

if (!config.init) {
  init();
  saveToConfig(config);
}

function saveToConfig(newConfig) {
  console.log("Save to config", newConfig);
  fs.writeFile(
    "./config.json",
    JSON.stringify(newConfig, null, 2),
    function writeJSON(err) {
      if (err) return console.log(err);
      console.log(JSON.stringify(newConfig, null, 2));
      console.log("writing to ./config.json");
    }
  );
}

function updateConfig(key, value) {
  config[key] = value;
  saveToConfig(config);
}

function init() {
  console.log("Init func", config.BASE_PATH);
  const filePath = filePathFormat();
  config.BASE_PATH = BASE_PATH;
  config.DOCUMENTS_PATH = BASE_PATH + filePath + "Documents";
  config.APPLICATION_PATH = BASE_PATH + filePath + "Applications";
  config.IMAGES_PATH = BASE_PATH + filePath + "Images";
  config.ARCHIVES_PATH = BASE_PATH + filePath + "Archives";
  config.init = true;

  config.DOCUMENT_EXTENSIONS = [".docx", ".doc"];
  config.APPLICATION_EXTENSIONS = [".msi", ".exe"];
  config.IMAGES_EXTENSIONS = [".png", ".jpg", ".jpeg"];
  config.ARCHIVE_EXTENSIONS = [".zip", ".rar", ".7zip"];
}

async function loadConfigData() {
  let rawdata;
  try {
    rawdata = fs.readFileSync("config.json");
  } catch (err) {
    if (err.code === "ENOENT") {
      // Create a new config file
      fs.writeFile("config.json", "{}", async function (err) {
        if (err) throw err;
        console.log("Config File is created successfully.");
        rawdata = await fs.readFileSync("config.json");
      });
    }
  }
  return JSON.parse(rawdata);
}

function setDownloadPath(path) {
  console.log("Path", path);
  let downloadFolder;
  if (process.platform === "win32") {
    downloadFolder = path + "\\Downloads";
    isWindows = true;
  } else {
    //Todo: add path for linux
    downloadFolder = path + "/Downloads";
  }

  return downloadFolder;
}

module.exports = {
  init,
  updateConfig,
  saveToConfig,
  config,
  setDownloadPath,
  filePathFormat,
};
