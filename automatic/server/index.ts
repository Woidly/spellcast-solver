import parse from "./args";

const ARGS = await parse();

console.log(`Starting server on port ${ARGS.port}`);
console.log(`Dictionary: ${ARGS.dictionary}`);
console.log(`Solver: ${ARGS.solver}`);

const CORS: ResponseInit = {
  headers: {
    "Access-Control-Allow-Origin": "*",
    "Access-Control-Allow-Methods": "GET, POST, OPTIONS, HEAD",
  },
};

Bun.serve({
  async fetch(req: Request): Promise<Response> {
    let date = new Date();
    console.log(
      `[${date.getHours().toString().padStart(2, "0")}:\
${date.getMinutes().toString().padStart(2, "0")}:\
${date.getSeconds().toString().padStart(2, "0")}] ${req.method} ${req.url}`
    );
    if (req.method.toUpperCase() != "POST") {
      return new Response(
        "Server is up! To solve the board, make POST request with 'board', 'swaps' and 'threads' query params",
        CORS
      );
    }
    let query = new URL(req.url).searchParams;
    let board = query.get("board");
    let swaps = query.get("swaps");
    let threads = query.get("threads");
    if (board && swaps && threads) {
      let text = await Bun.$`${ARGS.solver} -d ${ARGS.dictionary} -t ${threads} -f json -b ${board} -s ${swaps} 2>&1`
        .nothrow()
        .text("utf-8");
      try {
        let json = JSON.parse(text);
        return Response.json({ ok: true, data: json }, CORS);
      } catch (_) {
        return Response.json({ ok: false, error: text }, CORS);
      }
    }
    return Response.json({ ok: false, error: "Some of required params (board, swaps, threads) are missing" }, CORS);
  },
  port: ARGS.port,
});
