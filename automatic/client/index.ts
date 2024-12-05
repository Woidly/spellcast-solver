import { hookGame } from "./lib/gameplay";
import { UI } from "./lib/ui";

(function () {
  "use strict";
  hookGame();
  UI.showOverlay("UI is not implemented yet");
  UI.log("Hello, world!");
})();
