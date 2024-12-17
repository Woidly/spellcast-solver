import parse from "./args";

const args = await parse();

console.log(`Starting server on port ${args.port}`);
console.log(`Dictionary: ${args.dictionary}`);
console.log(`Solver: ${args.solver}`);

Bun.serve({
  fetch(req: Request): Response | Promise<Response> {
    return new Response("Hello World!");
  },
  port: args.port,
});
