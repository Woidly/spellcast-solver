// FIXME: This line breaks Prettier, formatting doesn't work in this file.
import style from "../style.css" with {type: "text"};

/**
 * Creates element of specified type, appends it to parent and returns it with proper type.
 * @param parent Parent element
 * @param tagName Element tag
 * @returns
 */
function createElement<T extends keyof HTMLElementTagNameMap>(
  parent: HTMLElement,
  tagName: T
): HTMLElementTagNameMap[T] {
  return parent.appendChild(document.createElement(tagName));
}

export const UI = new (class UI {
  overlay: HTMLDivElement;
  overlayText: HTMLSpanElement;
  overlayButton: HTMLButtonElement;

  constructor() {
    // Inject CSS.
    createElement(document.body, "style").textContent = style;
    // I wish they made framework-less JSX lol.
    let root = createElement(document.body, "div");
    root.className = "WS-root";
    // Main thingy
    let container = createElement(root, "div");
    container.className = "WS-container";
    let configContainer = createElement(container, "div");
    configContainer.textContent = "TODO: Add config";
    let statusContainer = createElement(container, "div");
    statusContainer.textContent = "TODO: Add status";
    let creditsContainer = createElement(container, "div");
    creditsContainer.className = "WS-credits";
    creditsContainer.innerHTML = "(c) 2024 <span>Woidly</span>";
    // Overlay
    let overlay = createElement(root, "div");
    overlay.className = "WS-centre WS-overlay";
    let overlayCentre = createElement(overlay, "div");
    overlayCentre.className = "WS-centre";
    let overlayText = createElement(overlayCentre, "span");
    overlayText.textContent = "Loading";
    let overlayButton = createElement(overlayCentre, "button");
    overlayButton.style.display = "none";
    // Assign all the stuff
    this.overlay = overlay;
    this.overlayText = overlayText;
    this.overlayButton = overlayButton;
  }

  hideOverlay() {
    this.overlay.style.display = "none";
  }

  showOverlay(text: string, buttonCallback: (() => void) | null = null, buttonText: string = "") {
    this.overlay.style.display = "block";
    if (buttonCallback) {
      this.overlayButton.style.display = "block";
      this.overlayButton.textContent = buttonText;
      this.overlayButton.disabled = false;
      this.overlayButton.onclick = () => {
        this.overlayButton.disabled = true;
        buttonCallback();
      };
    } else {
      this.overlayButton.style.display = "none";
    }
    this.overlayText.textContent = text;
  }
})();
