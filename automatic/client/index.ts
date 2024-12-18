import { hookGame } from "./lib/gameplay";
import { checkConnection } from "./lib/solver";
import { UI } from "./lib/ui";

// Not today, Discord analytics!
// @ts-ignore
unsafeWindow.Object.freeze(unsafeWindow.console);

function doCheckConnection() {
  checkConnection()
    .then(() => {
      hookGame();
      UI.showOverlay("Waiting for game hook...");
    })
    .catch(() => {
      UI.showOverlay("Connection to the server failed!", doCheckConnection, "Retry");
    });
}
UI.showOverlay("Waiting for server...");
doCheckConnection();
