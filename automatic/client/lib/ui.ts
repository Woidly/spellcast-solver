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
  console.log(handle, "is now draggable");
  handle.onmousedown = onMouseDown;
}

export const UI = new (class {
  root: HTMLDivElement;
  meta: HTMLDivElement;
  console: HTMLDivElement;
  overlay: HTMLDivElement;
  overlayTitle: HTMLSpanElement;

  constructor() {
    let root = document.body.appendChild(document.createElement("div"));
    Object.assign(root.style, {
      zIndex: 100,
      position: "absolute",
      top: "50vh",
      left: "vw",
      height: "200px",
      width: "300px",
      backgroundColor: "rgba(35, 35, 35, 85%)",
      transform: "translate(-50%, -50%)",
      userSelect: "none",
      color: "white",
      fontSize: "11px",
      padding: "5px",
      border: "4px solid black",
    });
    let meta = root.appendChild(document.createElement("div"));
    meta.innerText = "Metadata will be here";
    root.appendChild(document.createElement("hr"));
    let _console = root.appendChild(document.createElement("div"));
    Object.assign(_console.style, {
      fontFamily: "monospace",
    });
    let overlay = root.appendChild(document.createElement("div"));
    Object.assign(overlay.style, {
      zIndex: 101,
      position: "absolute",
      left: "50%",
      top: "50%",
      width: "100%",
      height: "100%",
      backgroundColor: "rgba(200, 200, 200, 85%)",
      transform: "translate(-50%, -50%)",
    });
    let overlayTitle = overlay.appendChild(document.createElement("span"));
    Object.assign(overlayTitle.style, {
      // top: 50%; left: 50%; position: absolute; transform: translate(-50%, -50%); font-size: large; text-align: center;
      position: "absolute",
      left: "50%",
      top: "50%",
      transform: "translate(-50%, -50%)",
      fontSize: "large",
      textAlign: "center",
    });
    overlayTitle.textContent = "Loading...";
    let drag = root.appendChild(document.createElement("div"));
    Object.assign(drag.style, {
      zIndex: 102,
      position: "absolute",
      right: 0,
      bottom: 0,
      padding: "2px",
      border: "2px solid black",
      cursor: "move",
    });
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
})();
