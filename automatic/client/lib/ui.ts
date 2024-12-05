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
  }
  handle.onmousedown = onMouseDown;
}

export const UI = new (class {
  root: HTMLDivElement;
  meta: HTMLDivElement;
  console: HTMLDivElement;
  overlay: HTMLDivElement;
  overlayTitle: HTMLSpanElement;

  constructor() {
    document.body.appendChild(document.createElement("style")).textContent = style;
    let root = document.body.appendChild(document.createElement("div"));
    root.className = "WSroot";
    let meta = root.appendChild(document.createElement("div"));
    meta.innerText = "Metadata will be here";
    root.appendChild(document.createElement("hr")).style.width = "100%";
    let _console = root.appendChild(document.createElement("div"));
    _console.className = "WSconsole";
    let overlay = root.appendChild(document.createElement("div"));
    overlay.className = "WSoverlay";
    let overlayTitle = overlay.appendChild(document.createElement("span"));
    overlayTitle.className = "WSoverlaytitle";
    overlayTitle.textContent = "Loading...";
    let drag = root.appendChild(document.createElement("div"));
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
    this.root = root;
    this.meta = meta;
    this.console = _console;
    this.overlay = overlay;
    this.overlayTitle = overlayTitle;
  }

  hideOverlay() {
    this.overlay.style.display = "none";
  }

  showOverlay(title: string) {
    this.overlayTitle.textContent = title;
    this.overlay.style.display = "block";
  }

  print(prefix: string, className: string, message: string) {
    let div = document.createElement("div");
    div.className = className;
    div.appendChild(document.createElement("span")).textContent = prefix;
    div.appendChild(document.createTextNode(" " + message));
    this.console.appendChild(div);
    if (this.console.scrollHeight - this.console.scrollTop - this.console.clientHeight <= 30) this.console.scrollTop = this.console.scrollHeight;
  }

  log(message: string) {
    this.print("LOG", "WSClog", message);
  }
})();
