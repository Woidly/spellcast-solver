// FIXME: This line makes Prettier think file has syntax errors, therefore skipping formatting.
import style from "../style.css" with { type: "text" };

function draggable(element: HTMLElement, handle: HTMLElement) {
  let oldx = 0;
  let oldy = 0;
  function onMouseDown(e: MouseEvent) {
    e.preventDefault();
    e.stopImmediatePropagation();
    oldx = e.x;
    oldy = e.y;
    document.onmousemove = onMouseMove;
    document.onmouseup = onMouseUp;
    handle.style.cursor = "grabbing";
  }
  function onMouseMove(e: MouseEvent) {
    e.preventDefault();
    e.stopImmediatePropagation();
    let dx = oldx - e.x;
    let dy = oldy - e.y;
    oldx = e.x;
    oldy = e.y;
    element.style.left = element.offsetLeft - dx + "px";
    element.style.top = element.offsetTop - dy + "px";
  }
  function onMouseUp(e: MouseEvent) {
    e.preventDefault();
    e.stopImmediatePropagation();
    document.onmousemove = null;
    document.onmouseup = null;
    localStorage.setItem("WSoldpos", JSON.stringify([element.offsetLeft, element.offsetTop]));
    handle.style.cursor = "grab";
  }
  handle.onmousedown = onMouseDown;
}

export const UI = new (class {
  root: HTMLDivElement;
  tabButtons: HTMLButtonElement[];
  tabs: HTMLDivElement[];
  console: HTMLDivElement;
  overlay: HTMLDivElement;
  overlayTitle: HTMLSpanElement;
  status: HTMLDivElement;
  statusText: HTMLSpanElement;
  statusButton: HTMLButtonElement;

  constructor() {
    document.body.appendChild(document.createElement("style")).textContent = style;
    let root = document.body.appendChild(document.createElement("div"));
    root.className = "WSroot";
    // Tab Buttons
    let tabButtonsContainer = root.appendChild(document.createElement("div"));
    let configTabButton = tabButtonsContainer.appendChild(document.createElement("button"));
    configTabButton.textContent = "Config";
    configTabButton.onclick = this.switchTab.bind(this, 0);
    let logsTabButton = tabButtonsContainer.appendChild(document.createElement("button"));
    logsTabButton.textContent = "Logs";
    logsTabButton.onclick = this.switchTab.bind(this, 1);
    let aboutTabButton = tabButtonsContainer.appendChild(document.createElement("button"));
    aboutTabButton.textContent = "About";
    aboutTabButton.onclick = this.switchTab.bind(this, 2);
    let tabButtons = [configTabButton, logsTabButton, aboutTabButton];
    // Split
    root.appendChild(document.createElement("hr")).style.width = "100%";
    // Tabs
    let configTab = root.appendChild(document.createElement("div"));
    configTab.className = "WStab";
    configTab.appendChild(document.createElement("button")).textContent = "TODO: put config here";
    let logsTab = root.appendChild(document.createElement("div"));
    logsTab.className = "WStab";
    let aboutTab = root.appendChild(document.createElement("div"));
    aboutTab.className = "WStab";
    aboutTab.innerHTML = "(c) 2024 Woidly (MIT license)<br>TODO: put more info here";
    let tabs = [configTab, logsTab, aboutTab];
    // Overlay
    let overlay = root.appendChild(document.createElement("div"));
    overlay.className = "WSoverlay";
    let overlayTitle = overlay.appendChild(document.createElement("span"));
    overlayTitle.className = "WSoverlaytitle";
    overlayTitle.textContent = "Loading...";
    // Status bar
    let statusBar = root.appendChild(document.createElement("div"));
    statusBar.className = "WSstatusbar";
    // Drag
    let drag = statusBar.appendChild(document.createElement("div"));
    drag.className = "WSdrag";
    drag.textContent = "Drag to move";
    let oldpos = localStorage.getItem("WSoldpos");
    if (oldpos) {
      try {
        let [x, y] = JSON.parse(oldpos);
        root.style.left = x + "px";
        root.style.top = y + "px";
      } catch {}
    }
    draggable(root, drag);
    // Status
    let status = statusBar.appendChild(document.createElement("div"));
    let statusText = status.appendChild(document.createElement("span"));
    statusText.textContent = "Loading...";
    let statusButton = status.appendChild(document.createElement("button"));
    statusButton.textContent = "Interrupt";
    // Init
    this.root = root;
    this.console = logsTab;
    this.tabs = tabs;
    this.tabButtons = tabButtons;
    this.overlay = overlay;
    this.overlayTitle = overlayTitle;
    this.status = status;
    this.statusButton = statusButton;
    this.statusText = statusText;
    this.switchTab(0);
  }

  switchTab(index: number) {
    let counter = 0;
    for (let button of this.tabButtons) {
      button.disabled = counter == index;
      counter++;
    }
    counter = 0;
    for (let button of this.tabs) {
      button.style.display = counter == index ? "block" : "none";
      counter++;
    }
  }

  hideOverlay() {
    this.overlay.style.display = "none";
  }

  showOverlay(title: string) {
    this.overlayTitle.textContent = title;
    this.overlay.style.display = "block";
  }

  hideStatus() {
    this.status.style.display = "none";
  }

  showStatus(status: string, callback: null | (() => void) = null) {
    this.status.style.display = "block";
    this.statusText.textContent = status;
    if (callback) {
      this.statusButton.disabled = false;
      this.statusButton.onclick = () => {
        this.statusButton.disabled = true;
        this.statusButton.onclick = null;
        callback();
      };
      this.statusButton.style.display = "block";
    } else {
      this.statusButton.style.display = "none";
    }
  }

  print(prefix: string, className: string, message: string) {
    let div = document.createElement("div");
    div.className = className;
    div.appendChild(document.createElement("span")).textContent = prefix;
    div.appendChild(document.createTextNode(" " + message));
    this.console.appendChild(div);
    if (this.console.scrollHeight - this.console.scrollTop - this.console.clientHeight <= 30)
      this.console.scrollTop = this.console.scrollHeight;
  }

  log(message: string) {
    this.print("LOG", "WSClog", message);
  }
})();
