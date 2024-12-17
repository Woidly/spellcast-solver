console.log("Starting server on port 27974");
Bun.serve({
  fetch(req: Request): Response | Promise<Response> {
    return new Response("Hello World!");
  },
  port: 27974,
});
