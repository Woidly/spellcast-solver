import { parseArgs } from "util";

async function locateArgs(params: [string, string[]][]) {
  const args = parseArgs({
    allowPositionals: true,
    args: Bun.argv,
    options: Object.fromEntries(params.map(([option]) => [option, { type: "string" }])),
  }).values;
  const results: Record<string, string> = {};
  for (let [option, defaults] of params) {
    const arg = args[option];
    if (arg && (await Bun.file(arg).exists())) {
      results[option] = arg;
    } else {
      for (let path of defaults) {
        if (await Bun.file(path).exists()) {
          results[option] = path;
          break;
        }
      }
      if (!results[option]) {
        throw new Error(`Failed to locate the ${option}`);
      }
    }
  }
  return results;
}

const CORS: ResponseInit = {
  headers: {
    "Access-Control-Allow-Origin": "*",
    "Access-Control-Allow-Methods": "GET, POST, OPTIONS, HEAD",
  },
};
const PORT = 27974;
const { dictionary: DICTIONARY, solver: SOLVER } = await locateArgs([
  ["dictionary", ["dictionary.txt", "../dictionary.txt", "../../dictionary.txt"]],
  [
    "solver",
    [
      "../../target/release/spellcast/solver",
      "../target/release/spellcast-solver",
      "target/release/spellcast-solver",
      "spellcast-solver",
    ],
  ],
]);
const THREADS = 12;

console.log(`Using ${SOLVER} as solver with ${DICTIONARY} dictionary`);
console.log(`Running HTTP server on port ${PORT}`);

Bun.serve({
  port: PORT,
  async fetch(request: Request): Promise<Response> {
    if (request.method.toUpperCase() != "POST") {
      return new Response(
        "Server is up! To solve the board, make POST request with 'board', 'gem_value' and 'swaps' query params",
        CORS
      );
    }
    let params = new URL(request.url).searchParams;
    let board = params.get("board");
    let gem_value = params.get("gem_value");
    let swaps = params.get("swaps");
    if (board && gem_value && swaps) {
      let text =
        await Bun.$`${SOLVER} -d ${DICTIONARY} -t ${THREADS} solver -f json -b ${board} -g ${gem_value} -s ${swaps} 2>&1`
          .nothrow()
          .text("utf-8");
      try {
        let json = JSON.parse(text);
        return Response.json({ ok: true, data: json }, CORS);
      } catch (_) {
        return Response.json({ ok: false, error: `Solver error\n${text}` }, CORS);
      }
    }
    return Response.json({ ok: false, error: "Some of required params (board, gem_value, swaps) are missing" }, CORS);
  },
});
