import style from "../style.css";

/**
 * Creates element of specified type, appends it to parent and returns it with proper type.
 * @param parent Parent element
 * @param tagName Element tag
 * @returns
 */
function createElement<T extends keyof HTMLElementTagNameMap>(
  parent: HTMLElement,
  tagName: T,
  className: string = ""
): HTMLElementTagNameMap[T] {
  let element = parent.appendChild(document.createElement(tagName));
  if (className) {
    element.className = className;
  }
  return element;
}

const DEFAULTS = {
  threads: window?.navigator?.hardwareConcurrency || 8,
  timer: true,
};

export const UI = new (class UI {
  threadsInput: HTMLInputElement;
  timerInput: HTMLInputElement;
  status: HTMLDivElement;
  statusText: HTMLSpanElement;
  statusButton: HTMLButtonElement;
  overlay: HTMLDivElement;
  overlayText: HTMLSpanElement;
  overlayButton: HTMLButtonElement;

  constructor() {
    // Inject CSS.
    createElement(document.body, "style").textContent = style;
    // I wish they made framework-less JSX lol.
    let root = createElement(document.body, "div", "WS-root");
    let container = createElement(root, "div", "WS-container");
    // Config
    let configContainer = createElement(container, "div", "WS-config");
    // -> Thread count
    let threadsLabel = createElement(configContainer, "label");
    let threadsInput = createElement(threadsLabel, "input");
    threadsInput.type = "number";
    threadsInput.min = "1";
    threadsInput.max = "255";
    threadsInput.step = "1";
    threadsInput.onchange = this.saveConfig.bind(this);
    threadsLabel.appendChild(document.createTextNode(" threads"));
    // -> Auto start timer
    let timerLabel = createElement(configContainer, "label");
    let timerInput = createElement(timerLabel, "input");
    timerInput.type = "checkbox";
    timerInput.onchange = this.saveConfig.bind(this);
    timerLabel.appendChild(document.createTextNode(" auto start timer?"));
    // Status
    let status = createElement(container, "div", "WS-status");
    let statusText = createElement(status, "span");
    statusText.textContent = "Loading...";
    let statusButton = createElement(status, "button");
    statusButton.style.display = "none";
    // Credits
    let creditsContainer = createElement(container, "div", "WS-credits");
    creditsContainer.innerHTML = "(c) 2024 <span>Woidly</span>";
    // Overlay
    let overlay = createElement(root, "div", "WS-status WS-overlay");
    let overlayText = createElement(overlay, "span");
    overlayText.textContent = "Loading";
    let overlayButton = createElement(overlay, "button");
    overlayButton.style.display = "none";
    // Assign all the stuff
    this.threadsInput = threadsInput;
    this.timerInput = timerInput;
    this.status = status;
    this.statusText = statusText;
    this.statusButton = statusButton;
    this.overlay = overlay;
    this.overlayText = overlayText;
    this.overlayButton = overlayButton;

    this.loadConfig();
  }

  loadConfig() {
    let threads = DEFAULTS.threads;
    let timer = DEFAULTS.timer;
    try {
      let threadsValue = localStorage.getItem("WS-threads");
      if (threadsValue) {
        let parsed = parseInt(threadsValue);
        if (!Number.isNaN(parsed) && 1 <= parsed && parsed <= 255) {
          threads = parsed;
        }
      }
      let timerValue = localStorage.getItem("WS-timer");
      if (timerValue) timer = timerValue == "true";
    } catch (e) {
      console.warn("Failed to load the config from localStorage", e);
    }
    this.threadsInput.value = threads.toString();
    this.timerInput.checked = timer;
  }

  saveConfig() {
    try {
      localStorage.setItem("WS-threads", this.threadsInput.value);
      localStorage.setItem("WS-timer", this.timerInput.checked.toString());
    } catch (e) {
      console.warn("Failed to save the config to localStorage", e);
    }
  }

  getThreads(): number {
    let threads = DEFAULTS.threads;
    let parsed = parseInt(this.threadsInput.value);
    if (!Number.isNaN(parsed) && 1 <= parsed && parsed <= 255) {
      threads = parsed;
    }
    return threads;
  }

  getTimer(): boolean {
    return this.timerInput.checked;
  }

  hideOverlay() {
    this.overlay.style.display = "none";
  }

  showOverlay(text: string, buttonCallback: (() => void) | null = null, buttonText: string = "") {
    this.overlay.style.display = "flex";
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

  hideStatus() {
    this.status.style.display = "none";
  }

  showStatus(text: string, buttonCallback: (() => void) | null = null, buttonText: string = "") {
    this.status.style.display = "flex";
    if (buttonCallback) {
      this.statusButton.style.display = "block";
      this.statusButton.textContent = buttonText;
      this.statusButton.disabled = false;
      this.statusButton.onclick = () => {
        this.statusButton.disabled = true;
        buttonCallback();
      };
    } else {
      this.statusButton.style.display = "none";
    }
    this.statusText.textContent = text;
  }
})();
