/**
 * Handles all things related to configuration of the app. Saving, loading and
 * settings the configuration variables is the job of this file and the
 * functions within it.
 *
 * @summary Configuration manager
 * @author
 *
 * Created at     : 2021-07-20 08:05:58
 * Last modified  : 2021-09-12 23:48:49
 */

const fs = require("fs");
let config = (async () => loadConfigData())();
console.log("Config loaded: ", config);

//TODO: Load this on start to Config
var downloadFolder = process.env.USERPROFILE + "\\Downloads";
if (config.BASE_PATH === undefined && config.BASE_PATH !== "") {
  BASE_PATH = downloadFolder;
} else {
  BASE_PATH = config.BASE_PATH;
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
  config.BASE_PATH = BASE_PATH;
  config.DOCUMENTS_PATH = BASE_PATH + "\\" + "Documents";
  config.APPLICATION_PATH = BASE_PATH + "\\" + "Applications";
  config.IMAGES_PATH = BASE_PATH + "\\" + "Images";
  config.ARCHIVES_PATH = BASE_PATH + "\\" + "Archives";
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

module.exports = {
  init,
  updateConfig,
  saveToConfig,
  config,
};
