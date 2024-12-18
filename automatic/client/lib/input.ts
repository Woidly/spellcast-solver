// TODO: Call in-game event handlers directly, it will make the whole thing more stable.

/**
 * Dispatches fake mousemove event with specified coords.
 * Very simple, just enough to fool the game.
 */
export function moveTo(target: EventTarget, x: number, y: number) {
  target.dispatchEvent(new MouseEvent("mousemove", { clientX: x, clientY: y }));
}

/**
 * Dispatches fake mousedown event.
 * No need to specify coords - game remembers them from last moveTo call!
 */
export function down(target: EventTarget) {
  target.dispatchEvent(new MouseEvent("mousedown", { button: 0 }));
}

/**
 * Dispatches fake mouseup event.
 * No need to specify coords - game remembers them from last moveTo call!
 */
export function up(target: EventTarget) {
  target.dispatchEvent(new MouseEvent("mouseup", { button: 0 }));
}
