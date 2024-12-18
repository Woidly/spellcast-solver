import { hookGame } from "./lib/gameplay";

// Not today, Discord analytics!
// @ts-ignore
unsafeWindow.Object.freeze(unsafeWindow.console);

hookGame();
