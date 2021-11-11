/**
 * Manager file is the place where the functionality and the configuration come together.
 *
 * @summary Manager file
 * @author
 * Created at     : 2021-07-20 08:07:52
 * Last modified  : 2021-09-05 21:45:49
 */

const path = require("path");
const fs = require("fs");
const moveFile = require("move-file");
const chokidar = require("chokidar");
const { config } = require("./config");

function watcher() {
  var watcher = chokidar.watch(BASE_PATH).on("all", (event, path) => {
    // console.log("init Watcher");
  });

  //Setup Watcher
  watcher
    .on("add", function (BASE_PATH) {
      console.log("File", BASE_PATH, "has been added");
      findAllMovableFiles();
    })
    .on("change", function (BASE_PATH) {
      console.log("File", BASE_PATH, "has been changed");
    })
    .on("unlink", function (BASE_PATH) {
      console.log("File", BASE_PATH, "has been removed");
    })
    .on("error", function (error) {
      console.error("Error happened", error);
    })
    .on("change", (BASE_PATH, stats) => {
      if (stats) console.log(`File ${BASE_PATH} changed size to ${stats.size}`);
    });

  return watcher;
}

function stopWatcher(watcher) {
  watcher.close().then(() => console.log("closed"));
}

function findAllMovableFiles() {
  fs.readdir(config.BASE_PATH, async function (err, files) {
    //Refacotr so its only one function
    const txtFiles = files.filter((el) =>
      config.DOCUMENT_EXTENSIONS.find(
        (extension) => path.extname(el) === extension
      )
    );

    const appFiles = files.filter((el) =>
      config.APPLICATION_EXTENSIONS.find(
        (extension) => path.extname(el) === extension
      )
    );

    const imageFiles = files.filter((el) =>
      config.IMAGES_EXTENSIONS.find(
        (extension) => path.extname(el) === extension
      )
    );

    const archiveFiles = files.filter((el) =>
      config.ARCHIVE_EXTENSIONS.find(
        (extension) => path.extname(el) === extension
      )
    );

    // Check if directory is available
    const PATHS = [
      config.DOCUMENTS_PATH,
      config.APPLICATION_PATH,
      config.IMAGES_PATH,
      config.ARCHIVES_PATH,
    ];
    PATHS.forEach((dir) => {
      if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir);
      }
    });

    //move app files
    appFiles.forEach(async (file) => {
      await moveFile(
        config.BASE_PATH + "\\" + file,
        config.APPLICATION_PATH + "\\" + file
      );
      console.log("Moved Apps");
    });

    // move text files
    txtFiles.forEach(async (file) => {
      await moveFile(
        config.BASE_PATH + "\\" + file,
        config.DOCUMENTS_PATH + "/" + file
      );
      console.log("Moved Text");
    });

    imageFiles.forEach(async (file) => {
      await moveFile(
        config.BASE_PATH + "\\" + file,
        config.IMAGES_PATH + "/" + file
      );
      console.log("Moved Images");
    });

    archiveFiles.forEach(async (file) => {
      await moveFile(
        config.BASE_PATH + "\\" + file,
        config.ARCHIVES_PATH + "/" + file
      );
      console.log("Moved Archives");
    });
  });
}

module.exports = {
  findAllMovableFiles,
  watcher,
  stopWatcher,
};
