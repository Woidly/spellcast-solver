import { hookGame } from "./lib/gameplay";
import { UI } from "./lib/ui";

(function () {
  "use strict";
  hookGame();
  UI.hideOverlay();
  UI.showStatus("Loading...");
  UI.log("Hello, world!");
})();
