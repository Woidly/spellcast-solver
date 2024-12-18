import { hookGame } from "./lib/gameplay";
import { UI } from "./lib/ui";

// Not today, Discord analytics!
// @ts-ignore
unsafeWindow.Object.freeze(unsafeWindow.console);

UI.showOverlay("Loading...");
hookGame();
